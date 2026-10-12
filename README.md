# Slot UI for BATLEXP G350, RG SP and RG35XX SP

Each current release includes a complete **fresh-install `.img.zip`**, source archive, instructions, manifest and SHA256 checksums. Extract the `.img` and flash it to the model shown below. The latest frontend, shaders and all eight systems—GB/GBC/GBA/SNES/PS1/N64/Dreamcast/PSP—are already included; no older image or follow-up update is needed.

| Device | Latest version and download | Game folder after first boot |
|---|---|---|
| BATLEXP G350 | [0.1.10](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.10) | `GAMES/Slot/Games` |
| Anbernic RG35XX SP | [0.2.2](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rg35xxsp-v0.2.2) | `BASEOS/Games` |
| Anbernic RG SP | [0.3.0](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rgsp-v0.3.0) | `BASEOS/Games` |

The RG SP and RG35XX SP are different models with separate boot images. G350 SYSTEM updates cannot be installed on either. Read the release's **START-HERE** before flashing: back up the card, use TF1/OS, keep TF2 empty initially, and let first-boot expansion finish before copying games. Flashing erases the target card. These new fresh images pass filesystem, payload, installer and checksum checks; physical first boot still needs validation. Existing firmware updates remain available for working cards. The repository is private, so sign in to download.

The H700 ports retain Menu-hold quit, save/load chords, shaders and D-pad/analog switching. See [RG35XX SP source/instructions](ports/rg35xxsp/README.md) and [RG SP source/instructions](ports/rgsp/README.md). RG SP uses its native 720×480 panel, round disc art and aspect-preserving console/GB display geometry. On either stickless handheld, **Menu+Select** switches D-pad digital/analog control for N64/DC/PSP; N64 starts analog. This maps full-deflection directions rather than providing analog precision.

Backup of the customized Slot frontend and firmware work for the RK3326 BATLEXP G350. The accepted stable baseline is **0.1.8**. **0.1.9.2** adds PSP/Dreamcast and repairs repeated Dreamcast launches. The owner confirmed that **0.1.9.3** substantially improves Dreamcast speed, with SoulCalibur almost full speed; sound still has dropouts. **0.1.9.4** targets those audio gaps with Dreamcast-specific buffering and pacing, corrects the PSP UMD shell and repairs PSP asset provisioning. Audible validation of 0.1.9.4 is pending on the handheld.

**G350 0.1.10** adds a Shader setting (Original, Sharp Pixels, Smooth, LCD, CRT)
and a full-height GB/GBC view that preserves the picture's aspect ratio. While
playing GB/GBC, L1 cycles between full-screen stretch and full-height with side
bars; R1 restores the old centered default. The scale is saved per game and the
shader is saved globally. See [the update guide](docs/G350-0.1.10-SHADERS.txt).
Existing drivers, emulator cores, game/state formats and quit controls are preserved.

Dreamcast's faster renderer, state format and relaunch repair are preserved. PSP worked well; its gameplay settings and the accepted SNES/N64/PS1 settings are unchanged. The Dreamcast buffer adds about 93 ms of target audio latency and cannot compensate for sustained emulation below real time. Do not install the superseded 0.1.9 desktop-build update.

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

Download the matching latest update ZIP from [Releases](https://github.com/nikunjsingh93/slot-ui-g350/releases). With the handheld off, replace **both** SYSTEM and SYSTEM.md5 on PAN4ELC. Safely eject and boot. Games live under `GAMES/Slot/Games`: `DC`, `PSP`, `N64`, `SNES`, `PS1`, `GBA`, `GB`, `GBC`. Keep a GDI's track files beside its GDI.

**Hold Menu alone for one second to quit to Slot.** On PS1/SNES/N64/DC/PSP, X/Y alone are game buttons; **Menu + X saves**, and **Menu + Y loads**. Game Boy systems use the different shortcuts listed below. PSP/DC automatic boot into a running game is disabled to avoid crash loops; manually launch from the shelf. DC VMU saves are under `Slot/Saves/DC`, and PSP memory-stick saves under `Slot/Saves/PSP/MemoryStick/PSP/SAVEDATA`.

For a fresh G350 card, flash **slot-g350-0.1.10.img.zip** from the latest release after extracting its `.img`. It already contains the latest SYSTEM pair. The old 0.1.3 image remains historical and requires an update if used. The latest image preserves the dedicated G350 base's boot/panel files and initial resize setup. RG35XX SP and RG SP use their own images and START-HERE instructions linked above.

## Controls and hotkeys

These are the current G350 0.1.10 controls, checked against the input/session code. **Menu** means the handheld's centre **M** button; L1/R1 are the first shoulder buttons and L2/R2 the second pair. For a combination, hold the first button, tap the second, then release both. A long hold of Menu **alone** quits the game.

### Save and load by system

| System / bundled emulator | Save a state while playing | Load the newest saved state |
|---|---|---|
| PS1 / PCSX ReARMed | Menu + X | Menu + Y |
| SNES / Snes9x 2005 Plus (or Snes9x) | Menu + X | Menu + Y |
| N64 / ParaLLEl N64 (or Mupen64Plus Next) | Menu + X | Menu + Y |
| Dreamcast / Flycast | Menu + X | Menu + Y |
| PSP / PPSSPP | Menu + X | Menu + Y |
| GBA / mGBA or gpSP | Select + R1 | Select + L1 |
| GB / mGBA | Select + R1 | Select + L1 |
| GBC / mGBA | Select + R1 | Select + L1 |

For Game Boy **Select** combinations, press the second button promptly, within about **0.6 seconds** of pressing Select. Holding Select longer hands it to the game instead. For example, on PS1: hold M, tap X, release both to save; hold M, tap Y, release both to load.

Save states are emulator snapshots, separate from a game's normal memory-card/VMU/memory-stick saves. Loading restores the saved moment. Use the browser below to choose an older state; the quick load shortcut chooses the newest saved state. Normal game saves remain managed through each game's own save menu.

### Common controls while playing

| Button / action | Result |
|---|---|
| Hold Menu alone for about 1 second | Save the resume data and return to the Slot shelf |
| Double-tap Menu quickly (within about 0.35 seconds) | Open the saved-state picture browser, if this game has saved states |
| Volume + / Volume − | Adjust the current speaker/headphone volume; holding repeats |
| Press both volume buttons together | Toggle mute; press them together again to unmute |
| Tap Power | Sleep / wake |
| Hold Power for about 1 second | Open Restart / Power off; select with Up/Down, confirm with A, cancel with B |
| PS1 only: Menu + R2 | Cycle to the next disc in a loaded multi-disc playlist |

For PS1 disc switching, launch the game's `.m3u` playlist with all discs listed, then use Menu + R2 when the game requests the next disc. A single-disc file has no next disc. PSP and Dreamcast do not have this disc-switch shortcut.

### Saved-state picture browser (all systems)

Double-tap Menu during gameplay to open it. Gameplay pauses while the browser is open.

| Button | Result |
|---|---|
| Left / Right | Choose a saved state |
| A | Load the selected state and resume |
| B, or double-tap Menu again | Close the browser and resume |
| X | Undo the most recent manual save/load, when offered (within 30 seconds) |
| Y | Delete the selected saved state |

### Extra Game Boy shortcuts (GBA / GB / GBC)

These apply while playing Game Boy systems. On PS1/SNES/N64/DC/PSP, the face, shoulder, Start and Select buttons go to the game instead; L2/R2 do **not** trigger rewind/fast-forward there, and Select combinations do **not** adjust display settings there. Return to the shelf for those settings.

| Button / combination | Result |
|---|---|
| Hold L2 | Rewind; release to play normally |
| Hold R2 | Fast-forward; release to play normally |
| Double-tap R2 quickly | Keep fast-forward on; tap R2 again to stop |
| Hold X / hold Y | Turbo A / turbo B |
| Select + Up / Down | Increase / decrease brightness |
| Select + Right / Left | Increase / decrease blue-light filtering |
| Select + Y | Toggle colour correction |
| GB/GBC only: L1 alone | Alternate full-screen stretch and full-height with aspect-preserving side bars |
| GB/GBC only: R1 alone | Restore the old centered default picture |
| GBA only: Select + Menu | Open Slot's link-play menu for a supported game/core; G350 multiplayer connectivity has not been validated |

Load-state, rewind and fast-forward are unavailable during an active link-play session. Select remains a normal game button when tapped on its own.

### Slot shelf and settings

| Button | Result |
|---|---|
| Left / Right | Browse games; holding continues scrolling |
| Up / Down | Jump to the previous / next starting letter |
| L1 / R1 | Switch system shelves |
| A | Launch the selected game |
| Tap Menu | Open settings; B or Menu closes them |
| Start on a GBA game | Choose mGBA / gpSP: Left/Right chooses, A keeps the choice, B cancels |

In settings, Up/Down selects a row, Left/Right changes its value, and A opens the selected item. The volume and Power controls above also work on the shelf. The Shader row selects Original, Sharp Pixels, Smooth, LCD or CRT; it is global and saved automatically. GB/GBC scaling is saved per game. The 0.1.10 SYSTEM update is required for these new options; no full reflash is needed.

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
