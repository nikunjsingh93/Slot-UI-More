# Slot for Anbernic RG35XX SP 1.1.16

Download [Slot RG35XX SP 1.1.16](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rg35xxsp-v1.1.16), extract **slot-rg35xxsp-1.1.16.img.zip**, and flash its `.img` to a 16 GB or larger card. This is one complete image; no older release or update is needed. Flashing erases the selected card.

Use TF1/OS and keep TF2 empty initially. Allow first-boot expansion/installation to finish, then shut down and add games to BASEOS/Games and matching PNG labels to BASEOS/Labels. Use only this model's image: its original BaseOS 1.3.0 boot/kernel/driver regions are retained.

Includes GB/GBC/GBA, SNES, PS1, N64, Dreamcast, PSP, native Ports, PortMaster and Artwork Studio v5. Native Rice is N64's default; highlight an N64 game and press Start for the per-game ParaLLEl fallback. The rejected Wave Race VBO trial is excluded. The Dreamcast improvements and accepted emulator binaries are shared with the tested RG SP setup.

Tap Menu at the shelf for Shader and per-system Screen Size (Default/Full). Hold Menu alone about one second to quit a game. Menu+Select switches D-pad/left-stick mode in N64/DC/PSP and native ports; native games remember their choice. Menu+X saves and Menu+Y loads console states; GB/GBC/GBA use Select+R1 / Select+L1. Ports use their own save menus.

The RG SP predecessor runtime was owner-tested. RG35XX SP's transferred changes and first boot of both freshly packaged images remain unverified on hardware. No commercial games, BIOS, APK/OBBs, private NFS/GTA executables or commercial artwork are included. Most Wanted Mobile was removed after failed trials.

See [H700 Ports/Wi-Fi/template instructions](../h700/README.md), [all hotkeys](../../README.md#controls-and-hotkeys), [release validation](../../docs/RELEASE-1.1.16.md) and [sharing/source notices](../../docs/SHARING.md). The common current H700 frontend is under `ports/h700/frontend`; model-specific launchers remain separate.
