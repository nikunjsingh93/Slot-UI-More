# Slot 1.1.17 — complete images for three devices

One complete flashable image for each exact model: BATLEXP G350, Anbernic RG SP
and Anbernic RG35XX SP. All report 1.1.17. Extract and flash your model's `.img`;
no previous firmware or follow-up system package is required. Flashing erases
the selected card. Finish first-boot expansion before adding your content.

## Added systems and artwork

| System | Folder inside `Games` | Bundled core |
|---|---|---|
| NES / Famicom cartridges | NES | FCEUmm |
| Mega Drive / Genesis | MD | Genesis Plus GX |
| Master System | SMS | Genesis Plus GX |
| Game Gear | GG | Genesis Plus GX |
| PC Engine / TurboGrafx-16 HuCards | PCE | Beetle PCE Fast |
| Arcade / supported Neo Geo games | ARCADE | FinalBurn Neo |

This brings Slot to fourteen emulated systems plus native Ports. Keep arcade
ZIP names unchanged and use sets matching the bundled FBNeo commit/DAT; arbitrary
MAME archives are not interchangeable. Neo Geo needs a matching BIOS supplied
separately. Sega CD/32X/Saturn, PCE CD/SuperGrafx and Famicom Disk System are not
part of this addition. See [detailed setup and controls](ADDITIONAL-SYSTEMS.md).

Artwork Studio handles all fourteen systems in one folder scan, including FBNeo
short-name aliases and BIOS filtering. Cartridge scans are preferred for the
new console systems; arcade uses logos/covers on an original PCB-style shell.
Existing labels are preserved by default. The standalone app is included under
Tools on all three cards and is also available through the project's Pages site.

Hold Menu alone to quit; console Menu+X saves and Menu+Y loads. Arcade Select
inserts a coin and Start starts. New digital consoles keep their D-pad/Select
input; H700's analog toggle remains for the supported 3D systems/native ports.
H700 per-system Default/Full screen settings include all added systems.

## Preserved behavior

The nine existing G350 core binaries and accepted N64/Dreamcast baseline are
retained byte-for-byte. H700 retains native Rice with per-game ParaLLEl fallback
on Start, Dreamcast audio fixes, PSP presentation, native Ports controls/volume,
GTA direct-EGL behavior, shaders and per-model panel handling. Model-specific
boot/kernel regions remain separate. The 117 unwanted legacy G350 ROM folders
remain excluded. Games, proprietary BIOS/APK/OBB files, saves, private game-linked
engines and downloaded commercial artwork are not bundled.

## Validation and limits

46 targeted Rust tests cover both frontends, system scanning/BIOS filtering,
cartridge geometry, portrait arcade rotation, controls, screen-setting persistence
and shader menus. Real NES/MD/SMS/GG/PCE cores pass original test-ROM video, PCM
and save-state roundtrips. FBNeo handles an invalid archive without a crash;
an actual arcade game has not been tested. Browser integration checks cover
matching, label dimensions, ZIP/direct saving and existing-label protection;
live artwork downloads succeeded for all six additions.

Complete-image validation checks model identity, exact payload hashes, original
boot regions, filesystems, embedded H700 BusyBox installation/retry/corruption
handling, G350 SYSTEM/kernel readbacks and ZIP/decompressed-image checksums.
The actual G350 tmpfiles rules still preserve user data and avoid the legacy
directories. Detailed records are under `docs/releases` and
[additional-system software validation](ADDITIONAL-SYSTEMS-VALIDATION.json).

**Fresh-image first boot, the newly added games, audio quality and performance
remain unverified on physical handhelds.** The owner-tested predecessor runtime
is not evidence that every new system/game works or runs full speed. RG35XX SP's
transferred runtime still lacks a documented physical test.

## Corresponding sources and licenses

`slot-1.1.17-sources.zip` contains the committed frontend/build/install sources
and exact native N64 source archives. `slot-1.1.17-additional-core-sources.zip`
contains all four newly bundled core source archives, checksums and build recipe.
`slot-1.1.17-compatibility-sources.zip` retains exact Debian sources/notices for
the unchanged SuperTuxKart libraries. Historical unchanged core/OS source records
remain linked under the retained 1.1.15 releases. No old firmware installation is
required to use these sources or images.

Preserve component licenses and source availability. Genesis Plus GX and FBNeo
include noncommercial terms; FBNeo also prohibits requesting donations for a
project using its source. H700 runtime notices remain applicable. The combined
firmware is not solely covered by Slot's GPL. See CREDITS.md and SHARING.md.
