# Slot for Anbernic RG35XX SP 0.2.0

Experimental single-card H700 port of the accepted G350 0.1.9.4 frontend.
Includes GB, GBC, GBA, SNES, PS1, N64, Dreamcast and PSP. Uses the official
model-specific BaseOS 1.3.0 RG35XX SP image and its H700 kernel, modules,
Mali fbdev driver, codec setup and power/lid support. The frontend uses the
device build, not the desktop windowing build. Hardware validation is pending.

Download `slot-rg35xxsp-0.2.0.img.zip` from the `rg35xxsp-v0.2.0` Release,
extract its `.img`, and flash a separate 16 GB or larger card. **RG35XX SP only**;
RG SP, RG34XX SP and G350 use different boot images. Read
[the installation/control guide](RG35XXSP-START-HERE.txt).

First boot must finish before copying games: BaseOS expands and reformats
TF1's data partition, then installs and verifies the bundled frontend. TF2
should remain empty. Copy the contents of the G350's `Slot/Games`, `Slot/Labels`,
`Slot/Saves` and `Slot/BIOS` to the corresponding folders at BASEOS's root.
Keep this image's `System`; G350 SYSTEM updates are incompatible.

**Hold Menu alone for one second to return to Slot.** Console save/load:
Menu+X / Menu+Y. N64/DC/PSP: Menu+Select toggles D-pad/left-stick control.
N64 starts in analog mode; DC/PSP start digital. This is full-deflection
directional mapping, with no analog precision. Controls, formats, artwork
and migration notes are in the guide. No games or proprietary BIOS are included.

The same pinned cores/settings preserve the G350's accepted fixes and states
where compatible. Cores are ARMv8 AArch64 and load in the SP userspace;
Cortex-A35 targeting is instruction-compatible with H700's Cortex-A53.
This release has not been benchmarked on an SP. DC/PSP/N64 performance varies
by game; do not assume full-speed Tekken 6 or every Dreamcast title.
The launcher requests stock performance governors without changing voltages
or adding an overclock. Original boot-resource/kernel regions are preserved.

## Rebuild on another computer

Clone this private repository while signed in. Download the official
[BaseOS RG35XX SP 1.3.0 archive](https://github.com/pvaibhav/BaseOS/releases/tag/v1.3.0)
and the existing G350 `slot-g350-0.1.9.4-audio-umd-update.zip` from
[v0.1.9.4](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.9.4).
The latter supplies only the known emulator binaries/assets/notices;
its G350 boot/kernel/driver files are excluded from the SP image.
Complete corresponding core sources remain in
[v0.1.9.2](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.9.2),
with the restore patches/scripts already in this repository. They need not be
uploaded again for this port. The new source ZIP includes the full SP frontend,
its overlay/build scripts and BaseOS build source.

From the repository root, use an AArch64 container (native ARM64 or Docker's
Linux ARM64 emulation). Example commands, with inputs mounted under `/inputs`:

```sh
docker build --platform linux/arm64 -f ports/rg35xxsp/Dockerfile -t slot-sp-build .
docker run --rm --platform linux/arm64 \
  -v "$PWD:/repo" -v "$PWD/inputs:/inputs:ro" -v "$PWD/dist:/dist" \
  slot-sp-build python3 /repo/ports/rg35xxsp/build_port.py \
  --repo /repo --work /tmp/slot-sp-rebuild --output /dist \
  --baseos /inputs/baseos-rg35xxsp-1.3.0.img.zip \
  --g350-update /inputs/slot-g350-0.1.9.4-audio-umd-update.zip
```

`build_port.py` restores shared source at `e787972`, applies `frontend.patch`,
verifies both input archives and builds with
`cargo build --locked --profile device --bin slot --no-default-features --features device`.
`package_sp.py` embeds an idempotent first-boot installer in rootfs p5, adds
Flycast's libgomp runtime, and checks rootfs readbacks, e2fsck, GPT, unchanged
boot regions, ZIP CRC and image SHA256. No user SD device is opened.
Build dates/toolchain metadata may change binary hashes on a rebuild.

`check_cores.c` verifies exports and dependencies with `dlopen(RTLD_NOW)` inside
BaseOS's chroot. `test_installer.py` records the actual BusyBox regression setup
used during this release; its local paths are shown in the script. The manifest
and validation record document the delivered build. Container GPU/audio tests
do not establish audible quality, real game speed, hinge behavior or device boot.

Source references:
[Slot](https://github.com/BrandonKowalski/slot),
[BaseOS](https://github.com/pvaibhav/BaseOS).
