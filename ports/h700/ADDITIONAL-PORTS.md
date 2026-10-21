# RG SP additional native ports (2026-10-21)

This is a physical-card test installation, not a public flashable-image release.
The owner confirmed GTA SA's direct display fix; the five new games and separate
GTA SA volume fix still require on-device testing. Exact sources, versions,
package hashes and test limits are in `../rgsp/ADDED-PORTS-20261021.json`.

All launchers retain Slot's parent supervisor: hold Menu alone for one second
to quit, use the volume rocker, and toggle D-pad/left-stick with Menu+Select.
The added games use the existing canonical GO-Super virtual pad so SDL's H700
fixed-layout driver does not ignore synthesized ABS_X/Y input. Existing GTA SA,
NFS U2, native N64, generic SP and G350 paths are preserved. The input test
covers the new profiles and their persisted per-game digital override.

## Private installation layout

The usual H700 paths are unchanged: scripts in `ports/` and `ports/.ports/`,
game directories in `ports/.ports/<id>/`, descriptors in `Games/PORTS/`, and
PNG disc prints in `Labels/PORTS/` matching each launcher's stem.

- **Grand Theft Auto 3 / Vice City:** original classic PC data supplied locally.
  Keep executable game data out of Git and releases. Download the PortMaster-MV
  engine/resource package from the recorded catalogue URL; copy owned PC asset
  folders, then apply the matching engine's `gamefiles` resource overlay. Build
  nosro1/re3 branches `sdl2` and `miami-sdl2`, at the recorded commits, with their
  pinned submodules. CMake: `LIBRW_PLATFORM=GL3`, `LIBRW_GL3_GFXLIB=SDL2`,
  `LIBRW_FORCE_GLES=ON`, `RE3_SNES_PAD=ON` or `REVC_SNES_PAD=ON`, Release,
  C/C++ flags `-O3 -mcpu=cortex-a53`. Build in ARM64 Bullseye with SDL2/OpenAL/
  mpg123 development libraries. These GLES builds avoid the package's desktop
  OpenGL dependency. Native `userfiles/` saves stay in each game directory.
- **RVGL:** the current recorded PortMaster ZIP already includes cars, tracks,
  music and the ARM64 engine. Use `rvgl.arm64`, its ARM64 non-graphics libraries,
  native Mali EGL/GLES, GLES `Profile=3`, and the supplied single-stick profile.
  Set the actual panel size (RG SP 720x480), not a desktop-test resolution.
  Standardized SDL button bindings: A accelerate, B brake, X item, Y look back,
  Start pause, Select reset, L1 reposition, R1 camera.
- **SuperTuxKart:** mobile assets are bundled. Its package is a GLES build; its
  `--render-driver=gl` selects GLES when built with USE_GLES2. The launcher uses
  480x320 scaling with advanced lighting, shadows, SSAO and other costly effects
  disabled. It does not promise full-speed gameplay on H700. The original ZIP
  lacks libcurl and SQLite plus supporting libraries required on BaseOS.
  Include the compatible ARM64 Bullseye dependency closure in its own `libs/`;
  retain the card's native SDL, EGL/GLES and system C runtime. Check all added
  libraries require GLIBC 2.30 or earlier, and run `ldd -r` against the actual
  H700 runtime before installation. The recorded launch fix includes these
  library hashes and the successful engine `--help` check.
- **Most Wanted:** Pixelforge release 1.1.2, original owned Android 1.3.128 APK
  and OBB with exact upstream SHA256. Run eapx setup and prepare_audio on Linux
  before copying. Keep five extracted Android libraries, the verified OBB and
  prepared sound files private. Include a game-local ARM32 runtime: H700 stock
  SDL/Mali from the upstream ac-gc-h700 runtime; matching Bullseye ARM32 libc,
  loader, libm/libdl/pthread/rt/libgcc/libstdc++, and Bullseye libasound2. The
  upstream support folder's newer ALSA, C++ and udev libraries must not be mixed
  with the Bullseye loader. Replace its `libudev.so.0` alias (SONAME libudev.so.1)
  with Bullseye ARMHF libudev1 247.3-7+deb11u5, retaining the alias SDL loads.
  Test `SDL_Init` with VIDEO, JOYSTICK and GAMECONTROLLER: a dlopen-only probe
  misses SDL's dynamic udev loading. The old library reproducibly fails this
  test; the replacement passes with the private ARM32 loader under qemu-arm.
  Also export `SDL_JOYSTICK_DEVICE="${SLOT_PORT_REAL_EVENT:-/dev/input/event1}"`
  in the Most Wanted launcher. Stock ARM32 SDL's udev enumeration misses the
  pad on BaseOS; the supervisor maps this original event path to Slot's virtual
  controller in the child's namespace. SDL's explicit-device registration runs
  before the udev scan. Test controller count, mapping and open, not only init.
  The green-screen exit recorded after fixing udev was the GLES preflight
  followed by a deliberate no-controller exit; guest execution had not started.
  Compile `slot-no-grab.c` for ARM hard-float and launch
  through the private loader with a private library path. System loaders and
  GPU libraries are never replaced. This remains an upstream alpha; its
  pre-race car-selection issue and mouse/driving mode behavior still apply.

The common new-port GL setup is `firmware/ports/slot-h700-native-gl.sh`.
GTA III/VC use the accepted direct `SDL_BLITTER_DISABLED=1` presentation path.
Most Wanted uses its own ARM32 graphics path; it never loads ARM64 helpers.

## GTA SA audio only

The private Android wrapper replaces `ALSOFT_CONF` after the parent has set it.
`slot-gta-audio.c` pins that variable to Slot's `Config/ports-alsoft.conf`, whose
ALSA device is `slot_ports`. It leaves all rendering untouched, including the
owner-confirmed direct native EGL path and original game effects. Compile with
`gcc -shared -fPIC -O2 slot-gta-audio.c -ldl -o slot-gta-audio.so`. Preload only
for GTA SA. The standalone test uses real OpenAL/ALSA file output to verify
routing even when the application tries to install its own config.

## Redistribution and testing

No proprietary data, APK, OBB, extracted Android libraries, linked private NFS
adapters or commercial artwork was added to the repository by this installation.
Preserve each engine/runtime's own credits and licence terms. Public firmware
images remain unchanged pending hardware tests. Container render/startup checks
are compatibility checks, not handheld frame-rate or audio guarantees.
