# Slot UI for BATLEXP G350

Backup of the customized Slot frontend and firmware work for the RK3326 BATLEXP G350. The accepted stable baseline is **0.1.8**. **0.1.9.2** adds PSP/Dreamcast and repairs repeated Dreamcast launches. The user confirmed Dreamcast save states and repeated launches now work; **0.1.9.3** reduces Dreamcast framebuffer allocation, queues GLES3 readback and applies lighter rendering settings; physical performance validation is pending.

The user tested 0.1.8's SNES/N64/PS1 fixes and confirmed that PSP and Dreamcast launch on 0.1.9.1. PSP Virtua Tennis 3 worked well. Dreamcast Virtua Tennis was slow and crashed when reopened after PSP. 0.1.9.2 repairs the reproduced memory-lifecycle fault and reduces Dreamcast rendering work using frame skipping. Do not install the superseded 0.1.9 desktop-build update.

## Backed up here

- `slot/`: complete customized Rust frontend, input/audio/rendering code and tests.
- `patches/`: core patches, pinned core revisions and submodule inventories.
- `firmware/slot-start`: final boot/provisioning script, including the checksum-specific Flycast upgrade.
- `studio/`: current Artwork Studio v4 for SNES/PS1/N64/DC/PSP, plus editable source and a portable build script. GBA/GB/GBC use the original Cart Studio.
- `docs/`: release instructions, checksums, validation and stable-base record.
- `scripts/`: portable build/packaging scripts and the actual-executable startup check.
- GitHub Releases: matching SYSTEM update pairs, complete corresponding source archives, and a fresh-install base image. Large binaries live in Releases rather than Git history.

No games, private game-test media, saves or proprietary BIOS files are included. The repository remains private; download release assets while signed in.

## Artwork studio

Open `studio/Slot-Auto-Artwork-Studio-v4.html` in Chrome or Edge. Choose the Slot folder, let original cartridge/disc artwork download, then save ready labels. Dreamcast reads `Games/DC`; PSP reads `Games/PSP`. Existing labels are preserved unless replacement is enabled. Logos and covers provide fallbacks when scans are missing. See the studio guide for formats and scan adjustments. Rebuild it with `python studio/source/build.py`; no external Python packages are needed. Only the current HTML, guide and necessary source files are kept here; local screenshots, test media, caches and duplicate ZIPs are excluded.

## Using the update

Download the 0.1.9.3 update ZIP from [Releases](https://github.com/nikunjsingh93/slot-ui-g350/releases). With the handheld off, replace **both** SYSTEM and SYSTEM.md5 on PAN4ELC. Safely eject and boot. Games live under `GAMES/Slot/Games`: `DC`, `PSP`, `N64`, `SNES`, `PS1`, `GBA`, `GB`, `GBC`. Keep a GDI's track files beside its GDI.

**Hold Menu alone for one second to quit to Slot.** X/Y alone are game buttons; save/load uses Menu + X/Y. PSP/DC automatic boot into a running game is disabled to avoid crash loops; manually launch from the shelf. DC VMU saves are under `Slot/Saves/DC`, and PSP memory-stick saves under `Slot/Saves/PSP/MemoryStick/PSP/SAVEDATA`.

For a fresh card, flash the supplied 0.1.3 base image, allow its initial partition setup to finish, then shut down and apply the latest matching SYSTEM pair. The base image alone is an older release; the update is required. Follow `docs/G350-0.1.3-UPDATE.md` for the G350 panel variants if applicable. This repository is for the G350, not Anbernic H700 devices.

## Rebuild on another laptop

Install Git, [GitHub CLI](https://cli.github.com/) and Docker Desktop with Linux containers. Enable ARM64 emulation (Docker Desktop normally provides it). Clone this private repository and sign into GitHub CLI:

```powershell
git clone https://github.com/nikunjsingh93/slot-ui-g350.git
cd slot-ui-g350
gh auth login
gh release download v0.1.8 --repo nikunjsingh93/slot-ui-g350 --pattern "slot-g350-0.1.8-parallel-n64-update.zip" --dir backups
gh release download v0.1.9.2 --repo nikunjsingh93/slot-ui-g350 --pattern "slot-g350-0.1.9.2-sources.zip" --dir backups
docker build --platform linux/arm64 -t slot-g350-builder .
docker run --rm --platform linux/arm64 -v "${PWD}:/repo" slot-g350-builder sh scripts/build.sh
```

The script verifies the downloaded archives, extracts the preserved 0.1.8 SYSTEM, rebuilds the current frontend/Flycast/PPSSPP, retains the other known working cores, provisions PSP assets, and produces a matching update ZIP in `dist/`. It writes image files only, never a physical disk. Allow substantial disk space and time, particularly for PPSSPP and ARM64 emulation.

The source archives contain the pinned upstream core sources/submodules, retained older core source and patches, and upstream OS provenance. The small pinned zstd build recipes omitted by the historical source export are restored from `patches/source-restore` before configuring the cores. The Flycast Linux repair is supplied as a patch against the archived v2.5 source; PPSSPP's thread-stop repair is already present in its source archive. The older cores can be rebuilt using the archived recipes, but this latest build intentionally preserves their verified binaries from the stable SYSTEM. The firmware build does not include games or run game probes automatically. For local regression tests, supply your own legally obtained copies.

The original ARM64 builder used Debian 11 (glibc 2.31), CMake 3.18.4 and the project's Rust 1.96 toolchain. Building against newer host glibc can produce binaries the firmware cannot load; use the supplied ARM64 Bullseye build container.

## Validation and licensing

The repeated core test reproduced the original DC → PSP → DC failure, then passed six mixed sessions and Dreamcast state restore after the patch. Tests also covered generated stereo audio, gameplay video, context teardown, full Slot Menu-hold exit, packaged KMS/DRM entrypoint, dependency resolution, archive CRC/SHA256/MD5 and SD read-back. Container timings do not establish G350 frame rates. See `docs/` for precise evidence and limitations.

Slot is GPL-3.0; preserve its LICENSE and upstream attribution. Core licenses vary (Flycast/PPSSPP are GPL; mGBA is MPL). Retain the notices and corresponding source archives when redistributing firmware. The inherited PAN4ELEC/AmberELEC OS remains under its component licenses; its provenance and upstream source links are in the source archives and documentation. This is an independent device adaptation of [Slot](https://slot-cfw.fyi/), not an official Slot or PAN4ELEC release.
