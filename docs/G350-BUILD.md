# Building the G350 source

Use Git, GitHub CLI and Docker Desktop with Linux containers/ARM64 emulation. The repository and releases are currently private; sign in with an authorized account. The build uses ARM64 Debian Bullseye so its executables load on the firmware's older glibc.

```powershell
git clone https://github.com/nikunjsingh93/slot-ui-g350.git
cd slot-ui-g350
gh auth login
gh release download g350-v1.1.15 --repo nikunjsingh93/slot-ui-g350 --pattern "slot-g350-1.1.15-system-files.zip" --dir backups
gh release download g350-v1.1.15 --repo nikunjsingh93/slot-ui-g350 --pattern "slot-1.1.15-core-sources.zip" --dir backups
docker build --platform linux/arm64 -t slot-g350-builder .
docker run --rm --platform linux/arm64 -v "${PWD}:/repo" slot-g350-builder sh scripts/build.sh
```

This portable route preserves the verified earlier core binaries, rebuilds the current frontend/Flycast/PPSSPP from the archived pinned sources, compiles the port supervisor and input-sharing helper, and applies the Ports/splash overlay. It outputs a **rebuilt SYSTEM-only** update under dist/. It writes image files, never a physical disk. The new Ports overlay/build wiring has syntax/recipe checks; the entire multi-hour portable rebuild has not been rerun for this documentation update. The 0.1.14 complete-image release was built and verified separately using the accepted 0.1.10 cores unchanged.

The new early boot splash also requires the matching KERNEL pair from the 1.1.15 system-files archive. To rebuild that pair, supply the untouched original G350 KERNEL from the upstream base (the current release already includes a patched KERNEL), then run:

```text
python firmware/ports/patch_boot_splash.py ORIGINAL_KERNEL firmware/ports/visuals/slot-splash.png KERNEL
```

If the RGB PNG exceeds the embedded archive budget, first save it as a 64-color palette PNG without changing its size, and pass that file instead. The tool validates the ARM64 image and embedded archive, preserves kernel code/offsets and all unrelated initramfs data, and refuses an oversized replacement. Generate KERNEL.md5 containing the output's MD5 followed by two spaces and target/KERNEL. Do not reuse the original checksum after changing KERNEL. No need to repatch a KERNEL that is already updated.

For a frontend-only iteration on an extracted accepted 0.1.10 SYSTEM: build slot with the device feature, compile firmware/ports/portwatch.c with -lasound -lm and slot-no-grab.c with -shared -fPIC -ldl, then run patch_system.py with the extracted root and those three built paths. Repack with mksquashfs gzip, 131072-byte blocks. Never include a test's temporary files in the firmware root. Compare all accepted core hashes before release.

Tests: firmware/ports/test_portwatch.py and test_port_volume.py take the compiled supervisor path. test_port_audio.py optionally takes an extracted firmware root to test its ALSA library (only the absent kernel mixer is mocked). The Rust g350_disc integration test needs Linux with surfaceless EGL; it renders actual framebuffer geometry. Retain exact source/patches and component notices with distributed binaries.

PortMaster is a separate upstream download. Game assets, generated NFS guest code and its linked executable are not inputs to the firmware build; keep them outside this repository. See the private NFS rebuild recipe for the exact executable matching process.

The complete 0.1.14 image also bundles the pinned official PortMaster manager (2026.10.18-0059). Supply its ZIP through SLOT_PORTMASTER_ZIP for a local rebuild. The image-stage tool scripts/build_latest_images.py installs the matching SYSTEM/KERNEL pairs into a clean model-specific image and verifies readback/filesystems/archive hashes. The first-boot provisioner preserves an existing manager. Corresponding H700 native launcher/runtime preparation is documented under ports/h700.
