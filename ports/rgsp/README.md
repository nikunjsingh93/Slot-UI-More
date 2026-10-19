# Slot for Anbernic RG SP 0.3.1

Download the [complete 0.3.1 image](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rgsp-v0.3.1): **slot-rgsp-0.3.1.img.zip**. Extract its `.img` and flash it directly to a 16 GB or larger card. No earlier image or separate update is needed. Flashing erases the selected card.

Use RG SP TF1/OS, with TF2 empty initially. Allow first-boot expansion and installation to finish before adding games to BASEOS/Games and PNG labels to BASEOS/Labels. This image is only for RG SP; other models have different boot files.

It includes GB/GBC/GBA/SNES/PS1/N64/Dreamcast/PSP, the accepted nine emulator core binaries, shaders, aspect controls and the new Ports shelf with a bundled H700-compatible PortMaster runtime. All model-specific BaseOS 1.3.0 boot/kernel/driver regions are retained. H700 native Ports are experimental and await physical game/audio/quit testing; they are not a claim of universal port compatibility or performance.

**Already running 0.3.0 on this model?** Use `slot-rgsp-0.3.1-ports-update.zip` instead of flashing. Merge its System folder into BASEOS, replacing matching files; keep Games, Labels, BIOS, Saves, States and Config. The emulator cores remain the accepted versions. Do not use a G350 SYSTEM update or convert a different model's card with this ZIP.

Hold Menu alone for one second to return to Slot. Existing emulator save-state, shader and D-pad/analog controls are retained. Native ports use their own save menus; emulator save-state shortcuts do not apply.

Read the [full H700 setup/Ports/labels/Wi-Fi guide](../h700/README.md), the release START-HERE and [all emulator hotkeys](../../README.md#controls-and-hotkeys). Wi-Fi setup uses a copy of the bundled System/wifi-example.conf saved as Config/wifi.conf with your own network details.

The matching sources ZIP contains the full updated model frontend. Runtime pins, preparation recipe, Slot adapter patch and original notices are under ports/h700. Compile in ARM64 Bullseye; retain component licenses and historical corresponding core sources. The original baseline source/packaging record remains in [the prior release](https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/rgsp-v0.3.0) and Git history. This is an independent Slot adaptation, not an official Anbernic, BaseOS or PortMaster release.
