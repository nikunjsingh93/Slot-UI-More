# Slot 1.1.16 on RG SP and RG35XX SP

One complete flashable `.img.zip` per exact model:

- [RG SP 1.1.16](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rgsp-v1.1.16)
- [RG35XX SP 1.1.16](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rg35xxsp-v1.1.16)

Extract and flash the `.img` to a 16 GB or larger card. Flashing erases that
card. Use TF1/OS, keep TF2 empty initially, and finish first-boot expansion before
adding games. No older image or separate update is needed. Back up existing
games, labels, BIOS, saves and settings before flashing.

Both use the common current frontend in `frontend/`, with their own BaseOS
1.3.0 boot/kernel/drivers and launcher. RG SP is 720x480; RG35XX SP is 640x480.
Never interchange their images or copy G350 SYSTEM updates here.

## Systems, controls and display

Includes GB/GBC/GBA, SNES, PS1, N64, Dreamcast, PSP, Ports and PortMaster.
Native Mupen64Plus/Rice is N64's default. Highlight an N64 game and press
**Start** for the per-game ParaLLEl fallback. Backends use separate state
formats. The rejected Wave Race VBO trial is excluded. Hold Menu alone about
one second to quit; native Rice returns to the same N64 shelf/game.

Tap Menu at the shelf for **Screen Size: Default/Full**, saved per system.
Default preserves the model's intended proportions. PSP uses available width
while preserving aspect by default. Shader presets apply to libretro sessions;
native Rice and ports use their own renderer settings. GB/GBC L1 alternates
stretch/height fit, R1 restores centered default. See [hotkeys](../../README.md#controls-and-hotkeys).

## Games, artwork and Wi-Fi

After first boot, put games in `BASEOS/Games/<SYSTEM>/` and PNG labels in
`BASEOS/Labels/<SYSTEM>/`, with matching filename stems. Restore your own BIOS,
Saves and States if needed. [Artwork Studio online](https://nikunjsingh93.github.io/Slot-UI-More/)
and the bundled `Tools/Slot-Auto-Artwork-Studio-v5.html` cover the eight emulated
systems. Native Ports disc prints are added manually under Labels/PORTS.

Copy `System/wifi-example.conf` to `Config/wifi.conf`, enter network details,
and reboot. Check `Config/wifi.log` if needed. No credentials are bundled.
Open Ports → PortMaster to install supported games; internet is needed for
downloads/missing runtimes. Replacing the pinned H700 manager can break its
environment. A Windows EXE alone does not run as a native Linux port.

## Native Ports

Game directories live in `ports/.ports/mygame/`; copy the game's launcher to
both `ports/.ports` and `ports`. Slot scans visible top-level launchers at
startup and when returning from ordinary ports. N64 skips the scan for a faster
return. Remove a game's launcher and `Games/PORTS/<name>.port` when uninstalling.
Match `My Game.sh` with `Labels/PORTS/My Game.png`; use a centered square disc
print. Slot draws the round disc and hub. Commercial scans are not supplied.

**Menu alone for one second quits. Menu+Select switches D-pad/left stick** and
remembers the mode per launcher in `Config/port-input/<launcher>.sh.mode`.
The manager remains digital. Select alone passes through. Movement is full
stick deflection; no physical right stick or fine analog pressure exists.
Volume inherits Slot's speaker/headphone level; hold repeats, both buttons mute.
Native games use their own save menu, not Slot emulator states/shaders.

`System/PortTemplates/` contains compatible example launchers for GTA III,
Vice City, GTA SA, NFS U2, RVGL and SuperTuxKart. After installing the matching
game/engine and required data, copy its template to both launcher locations,
replacing that game's launcher. Keep exact names for named controller profiles.
Generic custom launchers are not overwritten automatically.

- GTA III/VC need matching native GLES engines and classic PC data; see
  [build and per-port requirements](ADDITIONAL-PORTS.md). Their executables/data
  are not bundled with firmware.
- GTA SA needs the compatible separately supplied wrapper and Android data.
  Its template uses the owner-confirmed direct-EGL fix and audio config guard.
- NFS U2 needs the matching separately supplied recompiled engine and PC data.
  Original 4:3 proportions are retained; Menu+X opens its R3 name keyboard.
- RVGL and SuperTuxKart use their PortMaster packages. SuperTuxKart's template
  uses the added libraries under `System/PortCompat/supertuxkart/`; notices,
  provenance and corresponding Debian sources are supplied separately.
- Most Wanted Mobile is excluded after failed trials.

Logs: Config/ports.log, Config/slot-ports/logs/PortMasterH700.txt and each game's
own log. Compatibility, sleep/lid handling and performance vary by port.

## Validation and sources

RG SP's predecessor N64/Dreamcast/PSP improvements, GTA SA flicker fix, NFS U2,
GTA III, Vice City, RVGL and SuperTuxKart were owner-tested. **RG35XX SP's
transferred changes and first boot of both new images still need hardware
validation.** See [release checks](../../docs/RELEASE-1.1.16.md).

Build `ports/h700/frontend` in ARM64 Bullseye with Rust:
`cargo build --locked --profile device --bin slot --no-default-features --features device`.
Use a separate Cargo target directory from G350; their same-named crates use
different backends. Native N64's build recipe/pins are in `n64-native/`, and the
source ZIP includes its exact source archives. The host is
`firmware/ports/slot-native-n64.py`. Image composition/installer checks use
`scripts/build_latest_images.py` with verified artifact paths/hashes.

The manager uses [nextui-portmaster-h700 v0.4.0](https://github.com/Logarythms/nextui-portmaster-h700/releases/tag/v0.4.0)
and [NextUI h700-rc11](https://github.com/pvaibhav/NextUI/releases/tag/h700-rc11)
libraries. Runtime pins, preparation script, adapter patch and notices remain
in this directory. The selected libraries retain noncommercial/MinUI-derived
terms; this combined firmware is not unrestricted GPL-only software. Preserve
component notices and corresponding sources. See [credits](../../CREDITS.md)
and [sharing notes](../../docs/SHARING.md). No commercial game files are supplied.
