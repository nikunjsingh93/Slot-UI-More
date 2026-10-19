# G350 installation, ports and labels

For BATLEXP G350 only. RG SP, RG35XX SP and R36S use different boot images. This guide describes the shared 0.1.13 update, which contains firmware and integration code but no commercial game files or disc scans.

## What Slot and PortMaster do

Slot is the home screen and launches emulators or native Linux ports. PortMaster is a separate upstream application that downloads supported ports and their runtimes. Slot automatically discovers their launch scripts. Some ports include a free game; commercial ports usually require files from a specific purchased PC/Android version. A Windows EXE alone is not a general-purpose executable on this device.

## First installation on a new card

1. Download **slot-g350-0.1.14.img.zip** from [the complete G350 release](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.14).
2. Extract its `.img` and flash it directly to a 16 GB or larger card. Flashing erases the selected card; back up anything you need and select the correct removable drive.
3. Put it in G350 TF1/OS with TF2 empty initially. Charge the device, boot, and allow partition expansion/restart and manager installation to finish.
4. Once Slot appears, shut down using the power menu. Reconnect the card to copy games and labels to GAMES. Cancel any Windows offer to format an unreadable Linux partition.

The image already includes the latest system, gray SLOT boot/splash, all eight emulated systems, Ports integration and automatic PortMaster installation. **Do not flash 0.1.10 or install 0.1.13 first.** It is one complete image. Individual games and additional downloadable port runtimes are separate.

## Updating an existing Slot G350 card

Download `slot-g350-0.1.14-update.zip` and `G350-0.1.14-manifest.json` from [0.1.14](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.14). The generic release is separate from the owner's personal artwork update.

With the handheld powered off, copy these **four files from the ZIP to the root of PAN4ELC**, replacing their existing copies:

```text
PAN4ELC/
  SYSTEM
  SYSTEM.md5
  KERNEL
  KERNEL.md5
```

Do not format the card and do not copy these files into GAMES. Keep the file names exactly as shown; Windows can hide extensions. The KERNEL pair replaces the early splash; replacing SYSTEM alone leaves that earlier boot image unchanged. Use this only with the matching G350 base kernel (original SHA256 `18f4ca8d5a0c32b7674d6aa83dcaf657e5ccfad0aa51ae6322bd047e5207da7b`) or the already updated 0.1.13/0.1.14 kernel.

For a checked Windows installation, download/clone this source repository and run its installer in PowerShell, adjusting drive letters and paths:

```powershell
& .\firmware\ports\install-update.ps1 `
  -ArchivePath .\slot-g350-0.1.14-update.zip `
  -ManifestPath .\G350-0.1.14-manifest.json `
  -BootDrive D -GamesDrive E
```

The script checks the G350 card labels, original kernel and archive hashes, flushes writes, and verifies direct card reads. It can require administrator access for direct verification. It does not erase the card or replace game/save folders. Use `-VerifyOnly` for a later read-back check. Do not bypass a checksum error: leave the card connected and investigate the copy/reader.

Safely eject and boot. Missing platform directories are created automatically. Your existing games, labels, settings and saves remain. For the unchanged early logo or a boot problem, first check that both kernel files and both system files came from the same release; also check charge and the reader/card.

## Games and folders

Copy supported uncompressed game formats into the following directories on GAMES. Supply your own legally obtained files. Keep a CUE/GDI and all of its referenced track files together. Avoid showing both an M3U and its individual discs as separate games if you only want the playlist entry.

| System | Folder under `Slot/Games/` | Recognized extensions |
|---|---|---|
| Game Boy | `GB` | `.gb`, `.gbc` |
| Game Boy Color | `GBC` | `.gb`, `.gbc` |
| Game Boy Advance | `GBA` | `.gba` |
| SNES | `SNES` | `.sfc`, `.smc` |
| PlayStation | `PS1` | `.chd`, `.cue`, `.m3u`, `.pbp`, `.exe` |
| Nintendo 64 | `N64` | `.z64`, `.n64`, `.v64` |
| Dreamcast | `DC` | `.chd`, `.gdi`, `.cdi`, `.cue`, `.m3u`, `.elf` |
| PSP | `PSP` | `.iso`, `.cso`, `.pbp`, `.chd`, `.elf` |
| Native ports | `PORTS` | `.port` descriptors; actual games go under `GAMES/ports/` |

Recognized does not mean every game or file variant is supported by its emulator. BIOS requirements depend on the system/core; no proprietary BIOS is supplied. N64/DC/PSP performance varies significantly by title. Emulator saves and states are under `Slot/Saves` and `Slot/States`; native ports have their own save locations, often inside their game folder or the Linux storage partition. Do not assume copying only Slot/Saves backs up native ports.

## Bundled PortMaster

On the complete 0.1.14 image, PortMaster is installed automatically when missing. Open it from the Ports shelf; an existing working manager is preserved. The following manual steps are only for repairing a missing installation or using an earlier update. The bundled catalogue permits offline browsing; downloads and missing runtimes still need internet.


1. Download **PortMaster.zip** from [the pinned upstream release](https://github.com/PortsMaster/PortMaster-GUI/releases/tag/2026.10.18-0059). This is the version used for this integration; future releases may need retesting. Do not use the muOS/TrimUI variant.
2. Extract its `PortMaster` directory into `ports` at the root of GAMES:

```text
GAMES/ports/PortMaster/pugwash
GAMES/ports/PortMaster/control.txt
GAMES/ports/PortMaster/pylibs.zip
```

Avoid accidentally nesting `PortMaster/PortMaster`. Keep all upstream package files and notices. Slot launches its GUI directly, so you do not need to replace the firmware with another distribution or run a separate installer script. The Slot firmware already includes its own launcher and required base Python/SDL integration.

3. Boot, switch to the Ports shelf with L1/R1 and select **PortMaster**. Initial library extraction can take a little time.
4. Connect the G350 to the internet using a supported network adapter to download catalogue information, games and runtimes. A fresh offline install may show an empty catalogue until it obtains catalogue data. The owner's populated offline cache is not bundled in this source repository. Browsing a previously cached list offline does not download missing games/runtimes.
5. Install a port, read its instructions, and copy any required files from your own matching game version. Exit PortMaster; Slot refreshes its Ports shelf automatically.

Upstream [installation instructions](https://portmaster.games/installation.html) and [FAQ](https://portmaster.games/faq.html) describe offline packages and runtime requirements. Their generic launcher paths differ by firmware; this build uses `GAMES/ports`. Preserve an existing working PortMaster directory/settings when updating; this guide's extraction step is for first installation.

## Add a port manually

Download the Linux handheld port package, not just a Windows installer. Extract its top-level `.sh` launcher and data directory into `GAMES/ports`:

```text
GAMES/ports/My Game.sh
GAMES/ports/mygame/...runtime files...
GAMES/ports/mygame/...your required game data...
```

Follow that port's exact file/version requirements. Reboot the handheld or return from PortMaster to rescan. Slot creates `Slot/Games/PORTS/My Game.port` containing `script:My Game.sh`. You normally do not create descriptors manually. Launchers must use ASCII letters/numbers, spaces, dots, underscores, hyphens or parentheses; names starting with a dot and paths/commands are rejected. Use a short name (at most 120 characters including `.sh`). Some ports need additional runtimes downloaded through PortMaster or scripts adapted to this firmware.

GTA SA, NFS U2 and PortMaster have built-in shelf entries. An entry is not evidence that its game files/runtime are installed. GTA needs the upstream ARM64 wrapper plus a clean Android 2.10 APK and matching OBBs supplied by the user; no APK, OBB or extracted library is distributed. NFS needs an exact supported executable/data set and matching native runtime. The owner's private matching NFS build works but is slower than the other layout's optimized build; it is excluded from releases. See [the private-build recipe](../firmware/ports/nfs/README.md). Copying arbitrary PC game files does not install these ports automatically.

To remove a manually added port, remove its `.sh` launcher and corresponding `Slot/Games/PORTS/<name>.port` descriptor. Back up its saves before removing its data directory. The scanner adds entries but does not prune stale descriptors.

## Labels and disc artwork

For every system, put a PNG in `Slot/Labels/<SYSTEM>` with **exactly the same filename stem as the game or shelf descriptor**:

```text
Slot/Games/SNES/Super Example.sfc
Slot/Labels/SNES/Super Example.png

Slot/Games/PS1/My Adventure.m3u
Slot/Labels/PS1/My Adventure.png

ports/My Game.sh
Slot/Games/PORTS/My Game.port
Slot/Labels/PORTS/My Game.png
```

Use the same capitalization, spaces and punctuation. Enable filename extensions in your file manager to avoid `.png.png`. For a multi-disc PS1 playlist, match the `.m3u` name. Restart the handheld after adding/replacing art so the shelf reloads it.

For Ports, PS1 and Dreamcast, a square image with the disc print centered (512 × 512 is suitable) works well. Slot clips Ports artwork to a circle and draws the hub; use disc-face artwork rather than a photographed case if you want that appearance. A missing Ports label uses the generic disc. PSP uses a UMD shell; SNES/N64 use cartridge shells.

For automatic SNES/PS1/N64/DC/PSP artwork, open `studio/Slot-Auto-Artwork-Studio-v4.html` in Chrome/Edge, choose the Slot folder, scan/download art and save. Read [the studio guide](../studio/README.md). GB/GBC/GBA use [Cart Studio](https://studio.slot-cfw.fyi/). Current automatic Studio support does not include Ports: add their PNGs manually. Downloading artwork does not grant redistribution rights; commercial scans are excluded from the sharing package.

## Controls and troubleshooting

Hold **M/Menu alone for one second** to return to Slot. In native ports use the game's own save menu before quitting; termination is not a promise that unsaved progress will be written. Volume buttons adjust the current speaker/headphone level, held buttons repeat and both together toggle mute. Other controls depend on the port. Third-party scripts that replace audio devices or grab input may need adjustment.

Emulated systems retain their existing hotkeys, state browser and shaders; see [all controls](../README.md#controls-and-hotkeys). Menu+X/Y and double-Menu save-state browsing do not apply to native ports.

If a port fails, check its required data/version/runtime, then `Slot/Config/ports.log` and that port's own log (for these two examples: `ports/nfsu2/log.txt`, `ports/gtasa/gtasa.log`). A top-level launcher is needed for automatic import. An empty fresh PortMaster catalogue usually needs network/catalogue setup; a listed port still needs installation. A slow heavy game is not proof that Slot is slow: native games run after Slot releases its graphics/audio resources. Do not overwrite working game files while experimenting with another port.
