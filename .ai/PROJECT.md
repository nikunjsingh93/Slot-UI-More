# Project knowledge

## Purpose and evidence

Slot UI More is an independent adaptation of Brandon Kowalski's Slot for BATLEXP
G350, Anbernic RG SP and Anbernic RG35XX SP. It provides cartridge/disc shelves,
emulators, native Ports integration and a browser artwork tool. Credit upstream
Slot, its contributors and Markus's upstream artwork; preserve component notices.

- Repository: https://github.com/nikunjsingh93/Slot-UI-More
- Artwork Studio: https://nikunjsingh93.github.io/Slot-UI-More/
- Main references: `README.md`, `CREDITS.md`, `docs/SHARING.md`.
- Release evidence: `docs/releases/`, `docs/RELEASE-1.1.16.md`.
- Accepted runtime evidence: `docs/RGSP-ACCEPTED-20261021.json` and
  `docs/G350-2026-10-19-N64-VALIDATION.json`.

This knowledge combines inspected source, checked-in release records and owner
reports from this conversation. It is not an exhaustive reconstruction of every
historical chat. HANDOFF.md records what was verified in the latest session.

## Stack and repository map

There is no application server or database. Firmware runs on ARM64 Linux.
Rust 2021 workspaces declare a minimum Rust version of 1.96; follow the checked-in
toolchain/build pins and use an ARM64 Bullseye-compatible build environment.

| Location | Role |
|---|---|
| `slot/` | G350 Rust frontend workspace; KMS/DRM/Mesa device path |
| `ports/h700/frontend/` | Separate shared RG SP/RG35XX SP Rust workspace; vendor fbdev EGL/GLES |
| `slot/crates/` and H700 counterpart | `slot` app/session/audio, `slot-gfx` rendering, `slot-input`, `slot-power`, `slot-retro` libretro bridge, `slot-store`, `slot-ui`, `slot-sfxcut` |
| `firmware/slot-start`, `firmware/ports/` | Startup, C input/volume/process supervisor, shell/Python launch/provision helpers |
| `ports/rgsp/`, `ports/rg35xxsp/` | Model launchers, installer and historical model records |
| `ports/h700/n64-native/` | Native Rice build recipe, source pins, patch and config checks |
| `ports/h700/launchers/` | Example native game launchers; engines/data supplied separately |
| `patches/`, `scripts/` | Core pins/patches, source restoration, building and image verification |
| `scripts/release-1.1.16/` | Actual composition/source-packaging record; cache paths need adjustment elsewhere |
| `studio/` | Standalone HTML/JavaScript artwork app and Python build script |
| `.github/workflows/artwork-studio-pages.yml` | Build/deploy Artwork Studio to GitHub Pages |
| `docs/releases/`, `docs/downloads.json` | Image/source hashes, test counts and release inventory |

The enclosing development workspace also holds `work/` caches and `outputs/`.
Those are not substitutes for checked-in source and may contain private inputs.
The Git repository is this folder, not the surrounding workspace root.

## Device boundaries and storage

| Model | Hardware/panel | User-visible Slot root |
|---|---|---|
| G350 | RK3326; 640x480; PAN4ELEC-derived base | `GAMES/Slot` on computer; `/storage/roms/Slot` on device |
| RG SP | H700; 720x480, 3:2; model-specific BaseOS 1.3.0 | BASEOS root; `/mnt/sdcard` |
| RG35XX SP | H700; 640x480, 4:3; its own BaseOS 1.3.0 | BASEOS root; `/mnt/sdcard` |

Boot/kernel/driver files are model-specific. Do not use an image or G350 SYSTEM
pair on another model. Fresh setup uses TF1/OS, TF2 empty initially, and must
finish expansion before games are copied. A universal cross-model image is not
the release design. Current release composition preserves original boot regions.

Under the Slot root: `Games/<SYSTEM>`, `Labels/<SYSTEM>`, `BIOS`, `Saves`,
`States`, `Config`, `System`, and optional wallpapers/tools. System codes are
`GB`, `GBC`, `GBA`, `SNES`, `PS1`, `N64`, `DC`, `PSP`, `PORTS`.
Labels use the exact game/launcher filename stem plus `.png`. Preserve normal
game saves separately from emulator state snapshots.

## Runtime architecture and implemented behavior

The frontend discovers shelves, reads config/artwork, renders with GLES and runs
emulators through the libretro bridge. Bundled cores include mGBA/gpSP,
Snes9x/Snes9x 2005 Plus, PCSX ReARMed, ParaLLEl/Mupen64Plus, Flycast and PPSSPP.
Keep PPSSPP's free fonts/shaders/flash0 assets during packaging.

Native ports execute outside the frontend after it releases device resources.
The H700 session launcher handles frontend exit status 85 and `Config/port.request`;
the parent `slot-portwatch` supervises input, volume and Menu exit. Its virtual
controller/mount namespace supports standard evdev/joydev reads without loading
a 64-bit interception library into a 32-bit game.

- Hold **Menu alone ~1 second** to quit. Preserve this instead of introducing
  emulator-specific quit combinations.
- Console state shortcuts: Menu+X save, Menu+Y load. GB/GBC/GBA: Select+R1 save,
  Select+L1 load, with the prompt Select chord window. Ports use game save menus.
- H700: Menu+Select toggles native-port D-pad/left stick and persists per launcher;
  the manager stays digital. Named GTA SA/NFS U2 layouts have additional mappings.
- H700 N64: native Rice default; **Start on the N64 shelf** selects Rice/ParaLLEl
  per game. Select must not switch the backend. Rice and ParaLLEl states differ.
- `Config/n64-emulator.ini` stores per-game backend choices;
  `Config/return-to-shelf` restores the selected shelf/game after native exit.
  Skip PortMaster rescans on native N64 return to avoid multi-second delays.
- H700 `Config/screen-size.ini` stores per-system Default/Full. Default preserves
  intended proportions; PSP fills available width with its aspect maintained.
- Shader presets: Original, Sharp Pixels, Smooth, LCD, CRT. These apply to the
  libretro display path, not native Rice/native port renderers.
- GB/GBC L1 alternates stretch and full-height aspect fit; R1 restores default.
- Disc geometry compensates for the 720-wide logical canvas on 640-wide panels.
  PSP artwork uses a physical-style UMD shell, not a bare CD.

H700 PortMaster uses a pinned NextUI adaptation and private library directories;
do not replace it with G350/KMSDRM libraries. Game files live under
`ports/.ports/<game>`; launchers are exposed in `ports/`. In 1.1.16, templates in
`System/PortTemplates` must be deliberately copied to both launcher locations
after the matching game/engine is installed. They are not automatic universal
fixes for every PortMaster download. SuperTuxKart compatibility libraries live
under `System/PortCompat/supertuxkart` apart from game data.

## Accepted fixes and approaches not to repeat

1. **Dreamcast:** accepted threaded Flycast audio/render bridge fixes recurring
   wave-like audio starvation. Owner reported G350 gameplay/audio flawless.
   Preserve the accepted core/queue behavior; desktop timing is not device FPS.
2. **N64:** optimized accepted ParaLLEl improved G350; native Rice substantially
   improved RG SP. Compatibility varies, hence per-game fallback. The RG SP
   0.3.5 Wave Race VBO experiment caused severe regression and was rolled back.
   Do not reapply `parallel-vbo-option.patch` or that trial's frontend settings.
3. **Accepted core anchors:** ParaLLEl SHA-256
   `169d618a2540eab59070cd57889bd90a19ee853f30d39c6d00952929fad8a7c9`;
   Flycast `a796659516812f23c6fb8ad575f927d66517f1f0c1624f43dc6f6f0f9eb541f4`.
   Use full per-release manifests for the rest; do not select the newest-looking
   cached binary. Reject N64 states whose native layout size is incompatible.
4. **GTA SA H700 flicker:** owner-confirmed fix is GTA-only
   `SDL_BLITTER_DISABLED=1`, bypassing the rotating pixmap/blitter path for a
   direct native EGL window. Preserve original game effects. Opaque/postfx
   graphics interposers were unsuccessful and are not release dependencies.
5. **GTA volume:** a separate audio-only `slot-gta-audio.so` pins `ALSOFT_CONF`
   to Slot's controlled PCM. Real OpenAL routing passed; physical rocker
   confirmation for that specific last guard remains undocumented.
6. **NFS U2:** preserve the original compatible engine and natural 4:3 output.
   The owner rejected stretched output. H700 Menu+X supplies its R3 keyboard.
   A private engine/runtime must never be published with the firmware.
7. **Most Wanted Mobile:** removed at user request after failed ARM32 trials.
   Initial failures involved missing/incompatible runtime libraries, then input
   enumeration, finally unresolved `fstat` relocation. Do not reinstall it as
   accepted or confuse it with the 2005 game. Handoff contains no request to retry.
8. **SuperTuxKart:** failed for missing libcurl/dependency closure; compatible
   Bullseye libraries fixed startup and the owner confirmed it works. Validate
   transitive dependencies and retain exact corresponding sources/notices.
9. **Packaging:** an embedded installer referenced Studio v5 while the clean
   payload still carried v4; the image composer now installs v5 explicitly.
   Test actual BusyBox clean/retry/corrupt-install behavior, not only tar contents.
10. **Build caching:** reusing one Cargo target directory across the two
    same-named but different frontend workspaces caused incorrect dependency
    reuse. Build them in separate target directories.
11. **Hardware diagnosis:** a four-second G350 power-off once proved to be a
    low-battery issue, not firmware corruption. Check power before reflashing.

Some rejected prototype files remain untracked locally. Their presence is not
evidence of release use. Do not bulk-add or delete them without examining them.

## Build and verification entry points

Run Linux build commands in the appropriate ARM64 environment, from the stated
workspace. Read the relevant guide before downloading or composing images.

```sh
# G350 frontend, from repository root
CARGO_TARGET_DIR=/tmp/slot-g350-target cargo build --locked \
  --manifest-path slot/Cargo.toml --profile device --bin slot \
  --no-default-features --features device

# Common H700 frontend; intentionally a different target directory
CARGO_TARGET_DIR=/tmp/slot-h700-target cargo build --locked \
  --manifest-path ports/h700/frontend/Cargo.toml --profile device --bin slot \
  --no-default-features --features device

# Standalone artwork HTML; Python standard library only
python3 studio/source/build.py --output /tmp/Slot-Auto-Artwork-Studio-v5.html
```

- Full G350 source build: `docs/G350-BUILD.md`, `Dockerfile`, `scripts/build.sh`.
  Requires the guide's verified backup/source inputs; not a zero-input build.
- H700: `ports/h700/README.md`, `prepare_runtime.py`, runtime pins and native
  N64 recipe. `scripts/release-1.1.16/README.md` distinguishes actual cached
  staging inputs from portable build instructions.
- Image composition: `python3 scripts/build_latest_images.py --help`; requires
  an inputs JSON with checked paths/hashes. G350 SYSTEM packaging alone is not
  a full flash image.
- C supervisor: `gcc -O2 -Wall -Wextra -Werror firmware/ports/portwatch.c
  -o /tmp/slot-portwatch -lasound -lm`; Python input/volume tests take its path.
- Rust graphics tests need Linux EGL, typically `EGL_PLATFORM=surfaceless` and
  `LIBGL_ALWAYS_SOFTWARE=1`. Use production `--profile device` for timing-sensitive
  audio checks. Select component tests rather than assuming all tests lack ROM,
  hardware or GUI prerequisites.

## Artwork, distribution and scope

Studio v5 supports all eight emulated systems in one standalone HTML app. It
matches filenames, requests artwork online and writes labels locally through
Chrome/Edge folder access or exports ZIP. It does not upload ROM contents.
Ports art remains manual. Sources include LaunchBox, PSX1G1B1C1D and Libretro
thumbnails, with wsrv.nl for some browser-compatible image requests. See
`studio/ARTWORK-SOURCES.md`; downloaded art is not licensed for redistribution
merely because the app can fetch it. Edit `studio/source/` and rebuild the HTML.

Release convention: one synchronized version and one complete image per model,
e.g. `slot-rgsp-1.1.16.img.zip`, with consistently named GitHub release pages.
Firmware binaries belong in release assets; source belongs in Git. Keep hashes,
validation notes, credits and corresponding source archives accessible. Historical
source assets may still be required by currently distributed binaries.

The root project is GPL-derived, but component licenses differ, including
noncommercial terms for selected H700 runtime libraries. Do not describe the
whole image as unrestricted GPL-only software. Personal game files, private
generated code, BIOS, credentials, save backups and commercial scans stay out
of public Git/releases and these memory files.

R36S and G350 Android/mouse experiments appeared earlier in the conversation,
but are not targets of the current three-device Linux release. PC-to-Android
recompilation was discussed only; no new recompilation task was started.
