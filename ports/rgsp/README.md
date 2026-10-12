# Slot RG SP 0.3.0

## Complete fresh-install image

Download **slot-rgsp-0.3.0.img.zip** from
[rgsp-v0.3.0](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rgsp-v0.3.0).
Extract and flash its `.img` to a 16 GB or larger card, using RG SP TF1/OS.
Keep TF2 empty for the first boot. Allow expansion and Slot installation to
finish before copying games to BASEOS/Games and artwork to BASEOS/Labels.
The image already includes all eight systems, shaders and the current controls.
Back up existing games/BIOS/saves/settings first; flashing erases the card.
The matching source archive includes the complete frontend, BaseOS source,
fresh-image builder and conversion record. The fresh image passes filesystem,
installer/retry, payload, boot-region and ZIP checks. Physical first boot is pending.
The earlier conversion package is for migrating a specific inspected card;
use the `.img.zip` for future fresh installations.

Port of the complete RG35XX SP 0.2.2 frontend to the **new Anbernic RG SP**.
RG SP has a native 720x480 (3:2) panel; it is a separate model from RG35XX SP.
Uses official model-specific BaseOS RG SP 1.3.0. The boot0, bootloader/device
tree, kernel, drivers and rootfs come from that image, with libgomp1 added for
the existing Flycast core. No G350 or RG35XX SP kernel/driver substitution.

GB/GBC/GBA/SNES/PS1/N64/Dreamcast/PSP retain the same accepted nine AArch64
cores, PSP assets, audio settings, save-state behavior, shaders and controls.
Hold Menu 1000 ms to return to Slot. Menu+X saves; Menu+Y loads console
states. Menu+Select toggles D-pad/analog control on N64/DC/PSP. GB/GBC L1
cycles stretch/full-height aspect fit, R1 returns to default. RG SP console
picture widths preserve 4:3 and PSP's existing letterbox preserves 480:272.
Disc artwork uses the native canvas without the 640-pixel-panel correction.

The session's existing 64 GB card was backed up completely and all 1040 files
verified by SHA256 before conversion. Only the system ranges in
`boot-regions.json` and four staged frontend/guide files are updated. GPT,
expanded game partition, inactive OS slot and persistent OS userdata are kept.
This is a model conversion, not an ordinary System-folder-only update.

Apply `rgsp-0.3.0.patch` with `patch -p1` to the complete RG35XX SP 0.2.2
frontend (restore earlier SP frontend/shader/aspect patches first if building
from shared repository source). Build in AArch64 Debian Bullseye with Rust:
`cargo build --locked --profile device --bin slot --no-default-features --features device`.
The companion `slot-rgsp-0.3.0-sources.zip` includes the complete frontend,
BaseOS source, preparation/guarded install scripts and validation logs.
Preparation scripts document exact local input paths from this session.
The Windows installer is pinned to the inspected card; do not run it on an
uninspected disk. The conversion ZIP is not a flashable fresh-install image.

Official input: https://github.com/pvaibhav/BaseOS/releases/tag/v1.3.0
`baseos-rgsp-1.3.0.img.zip` SHA256
`39b184867c6628a39d0b197d70b06c07eb74fed2f3d54a1b27e372bac562df12`.

51 frontend unit tests, five scaling/panel/menu tests, nine core ABI/runtime
checks and a file-backed dry run of the actual range writer pass. Firmware
filesystem/readbacks are verified. Physical RG SP boot/play validation is
pending; emulator loading in a container is not a hardware benchmark.
No games, proprietary BIOS, artwork library, card backups or credentials
are included in source/package artifacts.
