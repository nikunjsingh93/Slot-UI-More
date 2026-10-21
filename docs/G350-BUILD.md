# Building the G350 source

For release 1.1.16 the frontend reports 1.1.16 and includes the N64 state-size
guard. The accepted emulator/core pins and 1.1.15 build-input downloads below
are retained. `scripts/package.py` produces developer SYSTEM files, not a full
flash image; `scripts/build_latest_images.py` composes the complete model image.
Use a separate Cargo target directory from the common H700 frontend. Current
release hashes are recorded under `docs/releases/`.

Use Git, GitHub CLI and Docker Desktop with Linux containers/ARM64 emulation. The repository and current releases are public. The build uses ARM64 Debian Bullseye so its executables load on the firmware's older glibc.

```powershell
git clone https://github.com/nikunjsingh93/Slot-UI-More.git
cd Slot-UI-More
gh auth login
gh release download g350-v1.1.15 --repo nikunjsingh93/Slot-UI-More --pattern "slot-g350-1.1.15-system-files.zip" --dir backups
gh release download g350-v1.1.15 --repo nikunjsingh93/Slot-UI-More --pattern "slot-1.1.15-core-sources.zip" --dir backups
docker build --platform linux/arm64 -t slot-g350-builder .
docker run --rm --platform linux/arm64 -v "${PWD}:/repo" slot-g350-builder sh scripts/build.sh
```

The default source build now rebuilds the accepted Flycast 2021 core with the ARM64 register repair, threaded PCM/input bridge and save/load fixes, plus the Cortex-A35/LTO ParaLLEl N64 core. Their upstream source archives are downloaded from pinned commits and checked by SHA256 before patching. PPSSPP is rebuilt from its existing archived source; the other base cores remain unchanged. All exact sources, patches and profiles are documented in [the runtime build notes](../experiments/g350-performance-20261019/README.md).

The core-only build helper has been run in the ARM64 builder and reproduced both accepted core SHA256 hashes exactly. The accepted binaries were separately tested by the owner on G350. The complete portable frontend/PPSSPP/overlay pipeline has not been rerun end to end for this source update, so newly rebuilt outputs still need normal validation. It produces a SYSTEM-only update in `dist/`; it never writes a physical disk. Public G350 1.1.15 image revision 3 packages the exact accepted frontend and these reproduced cores. Its filesystems, installed payload and archive hashes are checked; physical first boot of the newly packaged image remains pending. The performance-core-sources ZIP on that release contains both pinned upstream archives and the applied patches.

The new early boot splash also requires the matching KERNEL pair from the 1.1.15 system-files archive. To rebuild that pair, supply the untouched original G350 KERNEL from the upstream base (the current release already includes a patched KERNEL), then run:

```text
python firmware/ports/patch_boot_splash.py ORIGINAL_KERNEL firmware/ports/visuals/slot-splash.png KERNEL
```

If the RGB PNG exceeds the embedded archive budget, first save it as a 64-color palette PNG without changing its size, and pass that file instead. The tool validates the ARM64 image and embedded archive, preserves kernel code/offsets and all unrelated initramfs data, and refuses an oversized replacement. Generate KERNEL.md5 containing the output's MD5 followed by two spaces and target/KERNEL. Do not reuse the original checksum after changing KERNEL. No need to repatch a KERNEL that is already updated.

For a frontend-only iteration on an extracted accepted 0.1.10 SYSTEM: build slot with the device feature, compile firmware/ports/portwatch.c with -lasound -lm and slot-no-grab.c with -shared -fPIC -ldl, then run patch_system.py with the extracted root and those three built paths. Repack with mksquashfs gzip, 131072-byte blocks. Never include a test's temporary files in the firmware root. Compare all accepted core hashes before release.

Tests: firmware/ports/test_portwatch.py and test_port_volume.py take the compiled supervisor path. test_port_audio.py optionally takes an extracted firmware root to test its ALSA library (only the absent kernel mixer is mocked). The Rust g350_disc integration test needs Linux with surfaceless EGL; it renders actual framebuffer geometry. Retain exact source/patches and component notices with distributed binaries.

PortMaster is a separate upstream download. Game assets, generated NFS guest code and its linked executable are not inputs to the firmware build; keep them outside this repository. See the private NFS rebuild recipe for the exact executable matching process.

The complete 0.1.14 image also bundles the pinned official PortMaster manager (2026.10.18-0059). Supply its ZIP through SLOT_PORTMASTER_ZIP for a local rebuild. The image-stage tool scripts/build_latest_images.py installs the matching SYSTEM/KERNEL pairs into a clean model-specific image and verifies readback/filesystems/archive hashes. The first-boot provisioner preserves an existing manager. Corresponding H700 native launcher/runtime preparation is documented under ports/h700.
