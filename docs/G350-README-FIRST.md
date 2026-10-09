# Slot G350 — PS1 + SNES, experimental 0.1.0

This is a single flashable SD-card image for the **BATLEXP G350**, using the actual Slot frontend. It adds PlayStation discs and a Super Nintendo cartridge shelf. It is an independent experimental fork, not an official Slot, P4ELEC or BATLEXP release.

**This build has not booted on a physical G350.** Software tests and filesystem checks cannot establish that its screen, controls, sound, power handling or performance work on your unit. Keep your working original card and use a spare card for the first test. The image uses the dedicated P4ELEC G350 panel configuration; some G350 units need a different screen device tree.

## Flash and first boot

1. Extract `slot-g350-ps1-snes-0.1.0-experimental.img.zip`. It contains one `.img` file. The extracted image is about 2.2 GB.
2. Write that `.img` to a **spare microSD card of at least 16 GB**, using an image writer such as [Raspberry Pi Imager](https://www.raspberrypi.com/software/) with “Use custom”, or [balenaEtcher](https://etcher.balena.io/). Writing the image erases the selected card.
3. Put that card into the G350's **OS/TF1 slot**. For the initial test, leave TF2 empty and keep the original cards aside.
4. Boot with the battery charged. The base firmware expands storage, creates the GAMES partition and automatically reboots. Allow it to finish without removing power or the card.
5. Slot should then open. Complete its initial clock setup if shown. Its game shelves are initially empty.
6. Shut down through the frontend before removing the card. On your PC, open the **GAMES** partition and add your games under its `Slot` folder.

The first-boot process provisions the emulator cores from inside the image. There is no separate frontend ZIP to install or copy onto the card. The boot partition's `SYSTEM` file is the OS payload; you flash the complete `.img`, not that internal file.

## Where to put games

```text
GAMES/
  Slot/
    Games/
      PS1/
        Example.chd
        Multi Disc Game.m3u
        discs/
          Game (Disc 1).chd
          Game (Disc 2).chd
      SNES/
        Example.sfc
    BIOS/
      scph5501.bin
    Labels/
      PS1/
        Example.png
      SNES/
        Example.png
    Saves/
      PS1/
      SNES/
    States/
      PS1/
      SNES/
```

Use your own game dumps and compatible PS1 BIOS files. No PS1 BIOS or user game collection is supplied. Ordinary SNES cartridges do not require a BIOS.

- **PS1:** `.chd`, `.cue` with its referenced `.bin` tracks, `.m3u` playlists, and `.pbp`. Keep CUE/BIN files together and retain their matching names. BIN tracks are not separate shelf entries. CHD stores a disc image; it is still presented as a disc in Slot.
- **Multi-disc PS1:** put a playlist in `Games/PS1`, listing one relative disc path per line. For the example above, the playlist contents are:

  ```text
  discs/Game (Disc 1).chd
  discs/Game (Disc 2).chd
  ```

  A playlist appears as one game. Referenced discs are hidden from duplicate shelf entries. Change discs when the game asks for the next disc.
- **SNES:** extracted `.sfc` and `.smc` files. ZIP archives and patched-ROM formats are not shelf entries in this release.
- **Artwork:** optional PNG files with the same filename stem as the game or playlist, under `Labels/PS1` or `Labels/SNES`. PS1 artwork is clipped to a circular disc with a spindle hole. Without artwork, Slot generates a title label.

GB, GBC and GBA shelves are also retained under `Games/GB`, `Games/GBC` and `Games/GBA`. Only PS1 and SNES received new emulator smoke tests for this G350 release.

## Controls

| Context | Control | Action |
|---|---|---|
| Shelf | Left / Right | Choose a game |
| Shelf | L1 / R1 | Change system shelf |
| Shelf | A | Insert and launch |
| PS1 or SNES game | Hold MENU | Eject and return to the shelf |
| PS1 or SNES game | MENU + X | Save a state |
| PS1 or SNES game | MENU + Y | Load the newest saved state |
| PS1 playlist | MENU + R2 | Switch to the next disc |
| General | Volume keys | Change volume |

PS1 uses the digital controller in this first release. The analogue sticks and DualShock-only functions are not implemented. SNES has four face buttons and L/R support. GBA-specific turbo and rewind shortcuts are disabled for PS1/SNES, so their game buttons remain available. PS1/SNES frame pacing respects the core's reported PAL/NTSC rate; physical timing and performance remain untested.

Saves and states are filed separately per system. The existing Slot save/resume mechanism is used. Do not abruptly remove power while saving. Use the frontend's power menu for the initial test; hardware power-button behaviour has not been verified.

## Screen variants and a blank display

The active boot file is `rk3326-rg351mp-linux.dtb`, unchanged from the dedicated P4ELEC G350 image. The March 15 system update uses the same kernel and active DTB.

[P4ELEC issue #42](https://github.com/Kegg1701/P4ELEC/issues/42) documents a G350 with a different panel. Both configurations are included on the boot partition:

```text
screen-alternatives/g350-default.dtb
screen-alternatives/g350-hwrev1.dtb
```

If your hardware is identified as the alternate configuration from that issue, power off, put the card in your PC, preserve the existing active DTB, and copy `g350-hwrev1.dtb` to the root of the boot partition **renamed to `rk3326-rg351mp-linux.dtb`**. Keep the default backup. Do not mix H700 or unrelated R36S device trees into this image. A blank display alone does not prove which revision you have; your original stock DTB is the best evidence.

For a frontend startup failure, check `GAMES/Slot/slot.log`. A `KMSDRM not available` error on an actual device points to display access, driver or initialization problems; it is expected when this firmware is run in a container without the handheld's DRM hardware.

An optional base EmulationStation fallback is retained for diagnosis: create an empty `GAMES/Slot/Config/use-emulationstation` file and reboot. Remove it to return to Slot. This does not undo the fork's boot integration or restore all upstream services. It is not hardware-tested either.

## What was verified

See `G350-VALIDATION.md` for the exact checks and limitations. Highlights include the frontend regression suite, strict compiler lint checks, complete-frame GLES rendering, real ARM64 PS1 and SNES emulator tests, and a real SNES launch/save/resume cycle through Slot. The final image is checked independently after packaging.

## Foundation and source

- [Slot 1.4.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.4.0), base commit `d249d7731e52fabf3272119d4d3fd4702107951a`; frontend license GPL-3.0-or-later.
- [P4ELEC G350 releases](https://github.com/Kegg1701/P4ELEC/releases/tag/20250315), using the March 12 flash image's bootloader/kernel/DTB and the March 15 `SYSTEM` payload. P4ELEC is built on [AmberELEC](https://github.com/AmberELEC/AmberELEC).
- [PCSX ReARMed](https://github.com/libretro/pcsx_rearmed), commit `c8816799b50388e61cfe237fe2cdbb7d8175f20a`, GPL-2.0; ARM64 dynarec, NEON software GPU, CHD support.
- [Snes9x](https://github.com/libretro/snes9x), commit `fae2fea08f74180759ef540ee94259213f503480`. Its source archive includes its license terms; those include restrictions on commercial use.
- The retained mGBA/gpSP binaries come from the Slot 1.4.0 H700 distribution; their notices, source metadata and patches are included in the source bundle. They are ARM64 binaries, but this release does not establish hardware compatibility for every retained feature.

`slot-g350-ps1-snes-0.1.0-sources.zip` includes the modified frontend, new core sources, build/packaging scripts, verification logs and notices. It contains source for this fork and pointers to the upstream OS sources; it is not a complete source archive of every component in the P4ELEC distribution. Its original base kernel modifications and device-tree provenance are documented by [P4ELEC's maintainer](https://github.com/Kegg1701/P4ELEC/blob/main/AmberELEC/README.md).

This image does not include N64 or an R36S/RG35XX SP port. Do not flash it to those devices.
