# Slot for Anbernic RG35XX SP 0.2.2

## Latest complete image

Download **slot-rg35xxsp-0.2.2.img.zip** from
[rg35xxsp-v0.2.2](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rg35xxsp-v0.2.2).
Extract the `.img` and flash a 16 GB or larger card. It contains the current
frontend, all eight systems, shaders and GB/GBC scaling controls directly;
the older 0.2.0 image and separate shader/aspect updates are not required.
Read the release START-HERE. Finish first boot before adding games because
the initial expansion formats the empty data partition. Keep TF2 empty.
Flashing erases the card; back up games/labels/BIOS/saves/settings beforehand.
Fresh packaging retains the model-specific official boot/kernel regions and
passes filesystem, real BusyBox installer/retry and payload checks. Physical
first boot of this new image is pending. Historical port notes follow.

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

## Shader frontend update 0.2.1

For an existing SP 0.2.0 card, the 0.2.1 frontend adds a saved, global **Shader**
row to the shelf's Menu settings: Original, Sharp Pixels, Smooth, LCD and CRT.
Hold Menu for one second to return from a game, tap Menu, select Shader with
Up/Down, change it with Left/Right, and press B to return to the shelf.
Original preserves the previous GB/GBC/GBA LCD mask and sharp console output.
Smooth uses bilinear filtering. CRT adds lightweight scanlines, phosphor stripes
and edge shading. Effects apply to games and snapshots; shelf artwork stays clear.
These are built-in single-pass presets, without external shader-file support.
Existing colour correction and all game/quit/save/load controls are preserved.

See [the update instructions](SHADERS-0.2.1.txt) and
[build manifest](shaders-0.2.1-manifest.json). No full reflash is needed;
the update contains three System files and a guide, leaving the 0.2.0
first-boot installation marker, cores and user data in place.

To rebuild, first restore the SP 0.2.0 frontend using `frontend.patch` against
shared source `e787972`, then apply `shaders-0.2.1.patch` with `patch -p1`.
Run the same locked device-profile build described above. Copy its
`target/device/slot` to `System/slot` on the existing card; keep the SP launcher.
The shader source ZIP also contains the complete updated frontend and test logs.
GLES rendering, settings migration and menu checks pass in the build container;
panel appearance and speed still require checking on the handheld.

## GB/GBC aspect-ratio update 0.2.2

For an existing SP 0.2.1 card, the 0.2.2 frontend adds the same GB/GBC scaling
controls as G350 0.1.10. L1 alternates full-screen stretch and full-height with
the original 160:144 aspect ratio and centered black side bars. R1 restores the
previous default view. The scaling preference is saved per game. Shaders, GBA
shoulder buttons, console state controls, Menu-hold quit, the SP analog toggle
and all emulator cores/drivers retain their existing behavior.

See [the update guide](ASPECT-0.2.2.txt) and
[build manifest](aspect-0.2.2-manifest.json). Apply `aspect-0.2.2.patch` with
`patch -p1` to the complete SP 0.2.1 frontend and use the same locked device
build command above. The source ZIP also includes the complete updated frontend.
Install the matching frontend/launcher/manifest files on the existing card;
keep the original 0.2.0 installation marker. No full reflash is required.
Controls, saved preferences and GLES physical-panel pixel geometry pass tests;
handheld boot/display confirmation follows installation.
