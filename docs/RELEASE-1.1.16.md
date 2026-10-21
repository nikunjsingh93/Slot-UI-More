# Slot 1.1.16 — complete images for three devices

One full flashable image per exact model, with version 1.1.16 shown in Slot.
No older base or follow-up update is required. Model boot files are incompatible.

## Changes

- G350 retains accepted threaded Dreamcast audio, optimized N64 cores, Ports,
  gray SLOT splash and removal of 117 legacy folders. The frontend version is
  synchronized, and incompatible N64 state sizes are rejected safely.
- RG SP/RG35XX SP share the accepted H700 frontend: native Rice N64 default,
  per-game ParaLLEl selection on Start, accepted pre-VBO fallback, exact shelf
  return and faster exit. Dreamcast fixes, PSP width, per-system Default/Full,
  shaders and GB/GBC aspect modes use each model's panel handling.
- H700 native Ports gain per-game D-pad/left-stick mode, named GTA/NFS mappings,
  volume/Menu supervision and example launchers. GTA direct-EGL and audio config
  fixes are included. SuperTuxKart compatibility libraries are bundled apart
  from its game assets. Artwork Studio v5 installs on both H700 models.
- Most Wanted Mobile and rejected GTA graphics/Wave Race VBO trials excluded.
- No commercial games, BIOS, APK/OBBs, private game-linked executables, saves,
  profiles, credentials or downloaded commercial disc artwork included.

## Verification and limits

H700 checks cover both panel renderers, N64 Start/Select distinction, selection
and shelf restoration, GBA picker, shaders/aspect settings and Ports requests.
Input/volume tests cover both SP models, named GTA/NFS profiles, native N64,
held-key transitions, per-game persistence, Menu-only exit, mute and headphone
levels. Production-profile Dreamcast and SNES/PS1 audio-continuity tests pass.
An unoptimized audio timing run missed deadlines in the build environment;
no firmware timing change was made to hide it. G350 is built/tested separately
against its own backend.

Image checks cover exact core hashes, filesystem integrity, embedded payload
readbacks, official model identity and original boot-region preservation.
Real BaseOS BusyBox installer tests cover clean installation, idempotence,
corrupt payload rejection, interrupted retry and existing game preservation.
ZIP CRC and decompressed image SHA256 are checked. G350 retains original
bootloader/kernel/storage geometry and resize marker; its actual tmpfiles
rule creates none of the 117 unwanted legacy directories.

G350's performance baseline and the RG SP predecessor runtime were owner-tested.
**First boot of these newly composed images and RG35XX SP's transferred changes
are not hardware-tested.** Software checks do not certify game speed, audio,
sleep/lid behavior or universal PortMaster compatibility. Heavy scenes may lag.

## Sources

`slot-1.1.16-sources.zip` contains the release repository, full G350/common H700
frontends, native N64 component source archives, patches and build/install
scripts. `slot-1.1.16-compatibility-sources.zip` contains exact Debian sources
for the added SuperTuxKart libraries. Unchanged OS/core provenance and the three
1.1.15 core source archives remain available in that older release; no older
firmware installation is required. Retain component-specific licenses,
including H700 runtime noncommercial notices. See SHARING.md and CREDITS.md.
