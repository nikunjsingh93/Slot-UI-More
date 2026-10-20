# G350 N64 / Dreamcast trial — 2026-10-19

This branch is a hardware-test candidate, **not part of the public 1.1.15 revision 2 image**. The public revision only changes directory creation. Do not infer device FPS from host smoke-test timings.

## Why these changes

The owner's logs show missed audio deadlines and slow emulation despite stock CPU/GPU performance governors. Increasing an audio buffer alone cannot restore real-time emulation. RK3326 performance depends on game, renderer, dynarec and frontend overhead; newer emulator code is not always faster.

- ROCKNIX's [Flycast 2021 recipe](https://github.com/ROCKNIX/distribution/blob/f57baba03199b19bed20333cb0226944ec96b555/projects/ROCKNIX/packages/emulators/libretro/flycast2021-lr/package.mk) pins metallic77/flycast `603814c`. Use that lighter core with Cortex-A35 optimization, GLES2, 320x240, per-strip transparency and frameskip 1. Keep DSP off and HLE BIOS, matching the existing setup.
- Unlike modern Flycast, this legacy core's threaded mode delivers input/audio callbacks on its worker thread. Slot currently uses a thread-local callback host. Disable legacy threading, avoiding silent audio/input and a missing thread-wait callback at unload. Modern Flycast retains threaded mode. Identify the supported option set, rather than assuming every Flycast exposes the same options.
- Build current ParaLLEl N64 `6f23339`, ARM64 dynarec, Glide64 GLES2/HLE, 320x240. Set graphics accuracy to low, as in [ROCKNIX's profile](https://github.com/ROCKNIX/distribution/blob/f57baba03199b19bed20333cb0226944ec96b555/projects/ROCKNIX/packages/emulators/libretro/parallel-n64-lr/config/ParaLLEl%20N64.opt). Lower accuracy can introduce graphical differences; compare during testing.
- dArkOS also maintains RK3326-specific [Flycast builds](https://github.com/christianhaitian/rk3326_core_builds) and [standalone Mupen](https://github.com/christianhaitian/dArkOS/blob/daecf488075f0194c164578b90617fa7002ce8bd/build_mupen64plussa.sh). We retain libretro to preserve Slot's controls and long-press Menu exit. No overclock or new Vulkan driver is introduced.

## Exact core build

Use an ARM64 Debian Bullseye build environment with GCC 10, make, GLES development libraries and the Rust workspace dependencies. Download the two source archives described in the adjacent JSON files and verify SHA256 before extracting. No games, BIOS or saves belong in source packages.

For ParaLLEl, from its extracted source:

```sh
make -j4 platform=unix ARCH=aarch64 CPUFLAGS='-mcpu=cortex-a35 -DARM -DARM64 -DDONT_WANT_ARM_OPTIMIZATIONS' FORCE_GLES=1 HAVE_PARALLEL=0 HAVE_PARALLEL_RSP=0 HAVE_GLIDEN64=0 HAVE_GLIDE64=1 HAVE_RICE=1 GIT_VERSION=6f23339
```

For Flycast 2021, replace `cortex-a53` with `cortex-a35` in its Makefile, apply the adjacent ROCKNIX postprocess patch with `patch -p1`, then:

```sh
make -j4 platform=arm64_cortex_a53_gles2 FORCE_GLES=1 HAVE_VULKAN=0 HAVE_OPENMP=0 HAVE_LTCG=0 GIT_VERSION=603814c
```

Strip unneeded symbols from both .so files. Preserve the `reicast` option prefix (ROCKNIX's separate prefix rename is intentionally omitted). The adjacent patch comes from ROCKNIX's GPL-2.0-licensed package recipe. Upstream source licenses remain applicable.

Build the frontend from this branch's `slot/` with `cargo build --offline --locked --profile device --bin slot --no-default-features --features device`. In an extracted matching SYSTEM, replace `/usr/bin/slot-g350` and the two `/usr/share/slot/*_libretro.so` files. Existing cards also require replacing the matching two `Slot/System/` cores, since startup preserves existing files. Repack squashfs with gzip, block size 131072; compute SYSTEM.md5. Do not modify the kernel, bootloader or other cores.

## Validation and hardware hand-off

The `g350_compat_probe` example runs a caller-supplied game with temporary saves. It asserts nonzero video/audio, exercises Start, serializes/unserializes, unloads and repeats. Actual app options are used without test-only overrides. Diddy Kong Racing: 2 x 900 frames; Virtua Tennis: 2 x 900; SoulCalibur: 2 x 1800 passed on the build host. This is compatibility evidence, not a speed benchmark. The shipping frontend entrypoint also passed its KMS/DRM selection check.

The test card has an exact pre-trial SYSTEM/core rollback retained privately. Existing N64/Dreamcast state directories are renamed in place before testing; old states can be incompatible with changed cores. In-game save files remain, with private backups. Test both games from cold launch for around one minute, include actual gameplay, hold Menu to return, and relaunch. Reconnect the card to compare audio starvation/frame timing in Slot/slot.log. Public release promotion awaits that result.

## Follow-up 2: register corruption and N64 GPU waits

Hardware testing found N64 much improved, with dips in Diddy Kong Racing races. Dreamcast still had periodic missing audio and its performance log printed NaN averages and zero frame budgets.

The pinned Flycast 2021 ARM64 dynarec saved/restored s8-s15 (32-bit) at its mainloop boundary. [AAPCS64 requires the low 64 bits of v8-v15 to survive calls](https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#simd-and-floating-point-registers). Restoring s registers clears the upper half of the d register. This corrupts optimized callers' live double-precision values, including timing/audio calculations. Change only the eight Stp/Ldp instructions to d-register pairs; the existing 16-byte stack slots already fit them. Apply `flycast2021-aarch64-abi.patch` after the previous postprocess patch when reproducing the core build.

A diagnostic-only ARM64 assembly wrapper seeds d8-d15 with known doubles, calls the actual core retro_run with Slot callbacks active, captures all eight values, and restores the caller regardless of success. The pre-fix binary fails with all eight values zero; the corrected binary passes all eight. This test is enabled with Slot's `core-diagnostics` feature and adds no per-frame work to the normal shipping path. The optimized g350_compat_probe also validates real audio/video, state round trips and repeated launches for SoulCalibur and Diddy Kong Racing.

The N64 binary remains exactly the user's improved ParaLLEl build. The frontend now enables the bounded, nonblocking GLES3 pixel-pack queue for N64 when the driver supports it, with synchronous fallback. This avoids waiting for every GPU screen readback. It also answers libretro GET_AUDIO_VIDEO_ENABLE so this core can omit presentation work under audio pressure while audio stays enabled. Only one VI in three may be hidden, advancing the phase independently of audio pressure to avoid hiding every fresh frame of a 30-FPS game. This does not skip the guest CPU or all RDP work, and cannot guarantee full speed on demanding games. PSP and the other systems retain their previous paths.

The follow-up retains the same core settings, save formats, controls, kernel and stock clock limits. No new state-folder migration is needed. Test actual races and Dreamcast gameplay, then reconnect the card to check that timing diagnostics remain finite and starvation decreases. Do not publish hardware performance claims before this check.

## Follow-up 3: Dreamcast threaded PCM/control bridge

The owner's next hardware test confirmed Wave Race is smooth and Diddy improved. Keep that N64 core and frontend path unchanged. SoulCalibur still averaged about 26-29 ms per synchronous core call and accumulated audio starvation, so additional buffering could not restore real-time output.

ArkOS explicitly documents [RetroRun/RetroRun32 with Flycast](https://github.com/christianhaitian/arkos/wiki/ArkOS-Emulators-and-Ports-information#dreamcast). The maintained [RetroRun low-end profiles](https://github.com/navy1978/retrorun/tree/master/profiles/flycast2022-lowend) use threaded rendering and game/device-specific audio settings. Their benchmark results for other chips are not proof of G350 performance; the 32-bit core is not directly loadable by Slot's ARM64 process. We retain the validated 320x240/per-strip settings and use their architecture as a reference rather than copying higher-resolution profiles.

Apply `flycast2021-slot-callback-bridge.patch` in addition to the previous two core patches. It provides bounded, mutex-protected PCM delivery on retro_run, atomic snapshots of the four gamepads/analog controls, and an explicit capability marker. Slot enables legacy threading only for this marked core. The SH4 worker uses PCM-time pacing to replace the frontend callback backpressure normally supplied by RetroRun, preventing uncontrolled run-ahead. No speed-up/pitch conversion or time stretching is used. Queue overflow is logged and is a test failure. Frontend callbacks are invoked after releasing the PCM mutex.

Several old-core assumptions needed fixing: unsupported GET_FASTFORWARDING must default to normal-speed audio; threaded state operations need a monotonic clock when the optional perf interface is unavailable; waiting for a running worker while holding its restart mutex deadlocks consecutive save/load; cold state resume must start the worker; and a stop latch must prevent restart during unload. Slot joins this marked core before destroying its rendering context. The existing ARM64 register repair stays applied.

Slot's large Dreamcast queue previously required 160 ms of new PCM after every underrun. Its recovery threshold is now 20 ms, while the 320 ms capacity/160 ms pacing target remain. Other systems retain their old recovery threshold. Regression tests check both short-gap recovery and switching back to an ordinary core.

The probe now checks continued audio after state load and accepts optional caller-owned state-in/state-out paths. Repeated SoulCalibur tests, cross-loading a state produced by the previous synchronous core (including cold resume before the first frame), and repeated Crazy Taxi 2 launches passed with nonzero video/audio, correct register preservation, no queue overflow and a clean process exit. These are host compatibility tests, not G350 FPS benchmarks. Save formats, Menu-hold exit and bindings are unchanged. The card installation is still a physical-test candidate; the public complete image is not replaced by this experiment.

## N64 follow-up after Dreamcast acceptance

The owner confirmed the threaded Dreamcast build flawless for gameplay and audio. `docs/G350-STABLE-BASE.json` now records that exact primary runtime and source commit. Keep its SYSTEM, kernel, Flycast binary, audio and control behavior when experimenting with N64.

ArkOS documents standalone Mupen64Plus as usually fastest but less convenient to remap. The owner's previous standalone trial was rejected for control changes and compatibility issues, so this trial remains in ParaLLEl/Glide64. The upstream [RK3326 ParaLLEl build script](https://github.com/christianhaitian/rk3326_core_builds/blob/master/scripts/parallel-n64.sh) and target64 patch inform device-specific compilation. On this newer core, disabling threaded Angrylion entirely leaves unresolved shared HLE helper symbols; retain the dependency rather than applying that older patch wholesale. It does not select Angrylion as the renderer.

Build the identical pinned `6f233392445dcef4188a321d4ba65a776f0e1672` source with the following command, then strip unneeded symbols:

```sh
make -j4 platform=unix ARCH=aarch64 CPUFLAGS='-mcpu=cortex-a35 -mtune=cortex-a35 -DARM -DARM64 -DDONT_WANT_ARM_OPTIMIZATIONS -DARM_FIX -DCLASSIC -flto=4' FORCE_GLES=1 HAVE_PARALLEL=0 HAVE_PARALLEL_RSP=0 HAVE_GLIDEN64=0 HAVE_GLIDE64=1 HAVE_RICE=1 HAVE_LTCG=1 GIT_VERSION=6f23339
```

Link-time optimization permits cross-file optimization. The renderer stays Glide64, 320x240, low accuracy/HLE; there is no new clock limit, lower-resolution hack, changed binding or audio-rate adjustment. Only `Slot/System/parallel_n64_libretro.so` is replaced for this hardware test. The accepted Dreamcast SYSTEM and Flycast file are verified unchanged. Keep the original N64 file for rollback. Future complete images must explicitly incorporate the candidate only after owner acceptance.

Compatibility checks: the baseline and candidate each ran two 1800-frame Diddy sessions starting from the same private owner state, with matching audio sample counts, state sizes and nonblack pixel counts, plus successful state round trips, post-load audio and ARM64 register preservation. Wave Race passed two 900-frame sessions. Diddy host times were 3.07/3.01 seconds baseline and 2.60/2.83 candidate; host timing is not a G350 FPS benchmark and no device speed claim follows from it. Race-area hardware testing remains necessary. No private games or states are included in source control.
