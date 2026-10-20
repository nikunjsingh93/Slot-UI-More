# Native Ports for RG SP and RG35XX SP

The complete **RG SP 0.3.1** and **RG35XX SP 0.2.3** images include Slot's Ports shelf, a model-aware Menu/volume supervisor, and a pinned H700-compatible PortMaster runtime. They retain the earlier eight emulated systems, nine core binaries, shaders, aspect controls and emulator hotkeys.

This is a new, experimental Slot integration. Both frontends compile; platform discovery, frontend/control/panel tests and actual BaseOS Python/HTTPS/SDL imports pass. H700 native-port graphics, sound, volume and quit still need physical testing. A successful container test is not a handheld performance measurement. Existing G350 native games are not automatically certified for these stickless devices.

## One-image installation

Download the image for the exact handheld:

- [RG SP 1.1.15](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rgsp-v1.1.15): `slot-rgsp-1.1.15.img.zip`.
- [RG35XX SP 1.1.15](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rg35xxsp-v1.1.15): `slot-rg35xxsp-1.1.15.img.zip`.

Extract the `.img` and flash it to a 16 GB or larger card. Flashing erases the selected card. Put it in TF1/OS; keep TF2 empty initially. Allow first-boot expansion and installation to finish before copying games. No older image or separate update is needed. Use the model's own image; RG SP, RG35XX SP and G350 boot files are incompatible.

For an existing matching Slot card, use the model's **ports-update ZIP** instead. Merge its `System` folder into the root of BASEOS, replacing matching files. Do not delete or format the card. Keep existing `Games`, `Labels`, `BIOS`, `Saves`, `States` and `Config`; keep the existing emulator cores. The update adds the frontend/launcher/helpers/runtime and a build manifest. Back up any existing manager settings if you installed a different PortMaster manually. This is an update within a model, not a card conversion between models.

## Wi-Fi for downloads

After the first boot, connect the card to your computer. Copy the bundled `System/wifi-example.conf` (also [available here](wifi-example.conf)) to **BASEOS/Config/wifi.conf**, then replace the two example values with your Wi-Fi name and password. Keep the quotes. Safely eject and reboot. Slot starts association and DHCP in the background, allowing up to 25 seconds for BaseOS's radio to appear. Check `Config/wifi.log` if it does not connect. A missing file leaves networking alone; no credentials are bundled. Keep your personal file private.

## Manager and game paths

Open **Ports → PortMaster** in Slot. The manager uses the bundled H700 graphics/Python environment; installing an unmodified upstream manager over it can remove required compatibility fixes. Its self-updater is intentionally disabled by the upstream H700 adaptation. The game catalogue and game downloads remain available. An internet connection is needed for downloads and missing runtimes; this package does not include all games or every runtime.

```text
BASEOS/
  System/
    slot
    launch_frontend.sh
    PortMasterH700/             bundled manager and private libraries
  ports/
    My Game.sh                 visible launcher
    .ports/mygame/             installed port and your required game data
  Games/PORTS/My Game.port     created automatically by Slot
  Labels/PORTS/My Game.png     optional square disc print
  Config/slot-ports/           manager/runtime user settings and logs
```

The H700 adaptation keeps installed game data in `ports/.ports` and exposes `.sh` launchers in `ports`. To install manually, extract the port's `.sh` and game directory into `ports/.ports`, copy its `.sh` to `ports`, and follow that port's instructions for your purchased data. The adapter rewrites standard PortMaster paths on first launch. Do not copy the G350's entire runtime folder onto these cards: it uses a different graphics/backend environment.

Slot imports top-level `.sh` files when it starts and after returning from a port. Use short ASCII names containing letters/numbers, spaces, dots, underscores, hyphens or parentheses, ending in lowercase `.sh`. For artwork, match the launcher's stem exactly: `My Game.sh` → `Labels/PORTS/My Game.png`. A centered, square disc-face PNG such as 512 × 512 works well; Slot clips it to a round CD. Restart Slot after replacing labels. Ports artwork is added manually; the current automatic Studio covers the emulated systems.

## Controls and limits

Hold **Menu alone for one second** to return to Slot. Use native games' own save functions; Slot emulator states and shaders do not apply. Volume buttons use Slot's saved speaker/headphone levels, with held repeat and both buttons together for mute. These paths have synthetic input tests; their physical H700 behavior is pending. The separate NextUI volume HUD/power helper and suspend-audio override are disabled so they do not compete with Slot's supervisor. Native-port lid/sleep behavior is not validated.

The handhelds have no analog sticks. The stable 1.1.15 full images keep Menu+Select limited to emulators. **A separate native-input test update is now available for RG SP and RG35XX SP; physical testing remains pending.** It replaces only `System/slot-portwatch` and that model's `System/launch_frontend.sh`. Do not mix models or replace the full System folder.

With that test installed, hold **Menu then press Select** inside a native game to switch D-pad / left stick. Release both buttons afterward. The default is D-pad; each launcher's choice is saved independently in `Config/port-input/<launcher>.sh.mode` (`0` digital, `1` analog). Rename a launcher and it receives a new default mode; remove its mode file while powered off to reset it. The manager always keeps ordinary controls. Select alone is passed through; Menu+Select is consumed and cannot trigger the Menu-alone quit timer. Hold Menu **alone** for one second to quit. Full stick deflection is simulated; there is no fine analog movement, mouse mode or right-stick mode.

The supervisor copies the built-in pad into a Linux uinput device, forwards face/shoulder buttons unchanged, and routes D-pad events to either hats or left X/Y axes. A private mount namespace exposes that copy at the original evdev/joydev paths only inside the port's process tree, hiding the duplicate node. This supports ordinary device reads without depending on a 32/64-bit game library preload. The host retains its original paths; grabs and the virtual device are released when the port ends. External controllers are not remapped. Kernel uinput and mount-namespace support are required; a failed setup returns to Slot before the game starts.

No GTA SA-specific mapping or binary patch is included, as requested. Individual keyboard-style ports may still need game-specific `.gptk` mappings for analog mode. Ports that bypass standard device paths or change namespaces require their own adaptation. No promise of full-speed NFS U2/GTA or universal PortMaster compatibility is made.

Build the test ZIPs with `python3 scripts/package_sp_native_input.py --outputs /outputs` in the ARM64 Bullseye builder. Run `firmware/ports/test_port_input.py`, `test_portwatch.py` and `test_port_volume.py` against the compiled supervisor. The separate `test_port_input_namespace.py` exercises actual kernel mount isolation using temporary files in a disposable privileged container. Synthetic input/volume and real namespace tests pass; creation of a uinput pad, controller enumeration and actual games on both handhelds remain unverified. Stable full-image downloads have not been replaced by these test packages.

Logs: `Config/ports.log` for the Slot supervisor and `Config/slot-ports/logs/PortMasterH700.txt` for the H700 adapter, plus the game's own log. The automatic scanner adds entries but does not remove stale ones; remove the `.sh` and its `Games/PORTS/*.port` descriptor when uninstalling a game. Save locations vary by port; back up its game directory and relevant runtime settings before deleting anything.

## Source and provenance

The runtime uses [nextui-portmaster-h700 v0.4.0](https://github.com/Logarythms/nextui-portmaster-h700/releases/tag/v0.4.0) and the H700 SDL libraries from [NextUI h700-rc11](https://github.com/pvaibhav/NextUI/releases/tag/h700-rc11), rather than G350/KMSDRM libraries. GPU/kernel drivers remain the official model-specific BaseOS drivers. `runtime-pins.json` records exact download hashes and the adapter source commit; `runtime-adapter.patch` records Slot's path/control changes. `prepare_runtime.py` verifies and stages those inputs, omitting optional game-specific binary fixes. Upstream recipes, notices and each bundled component's licenses must be retained when redistributing.

The matching source ZIP contains the full updated model frontend. Build it in ARM64 Bullseye with Rust using `cargo build --profile device --bin slot --no-default-features --features device`. Compile the common supervisor with `-lasound -lm`, and the input-sharing helper with `-shared -fPIC -ldl`. Add the runtime/launch files to the previous clean model payload, update its frontend manifest/hash and include PORTS in its system list. `scripts/build_latest_images.py` packages and verifies the image with the updated first-boot installer. It never writes a physical SD card.

The selected NextUI library build carries upstream noncommercial terms and MinUI-derived notices. The adapter is MIT, but those libraries keep their own terms; copies are in licenses/ and the packaged System/licenses/portmaster-h700 directory. See the repository sharing notes before redistribution.


## Optional Controls Test

`native-input-probe.c` is a small original SDL2 diagnostic with no game data. Build in ARM64 Bullseye with `gcc -O2 -Wall -Wextra -Werror native-input-probe.c -o slot-controls-test $(sdl2-config --cflags --libs)` (SDL2 development headers required). A `SDL_VIDEODRIVER=dummy ./slot-controls-test --self-test` smoke test runs five frames without hardware. That checks application startup/render calls, not physical input or the vendor video driver.

Install its binary in `ports/.ports/slot-controls-test/slot-controls-test` and copy the adjacent `Slot Controls Test.sh` to both `ports/.ports` and `ports`. Restart Slot to scan it, or add a `Games/PORTS/Slot Controls Test.port` descriptor containing `script:Slot Controls Test.sh`. No game, BIOS, Wi-Fi or additional download is required.

Open **Ports → Slot Controls Test**. Hold a direction: in digital mode the DPAD HAT number changes while LEFT STICK X/Y stay zero. Hold Menu, tap Select, release both and hold a direction again: X/Y should change while HAT stays zero. Check face/shoulder buttons, then hold Menu alone one second to return. Launch it again to check that its mode is remembered. The mode for this diagnostic is separate from every game. Reconnect the card after testing to inspect `Config/ports.log` and the H700 adapter log if anything fails. Actual on-device testing is pending.
