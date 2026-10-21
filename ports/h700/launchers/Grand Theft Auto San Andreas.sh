#!/bin/sh
# User supplies their own verified GTA SA runtime and Android 2.10 data.
GAMEDIR="${SDCARD_PATH}/ports/.ports/gtasa"
cd "$GAMEDIR" || exit 1
# H700 uses vendor fbdev/Mali SDL; KMSDRM/Wayland overrides are incompatible.
unset SDL_VIDEODRIVER DISPLAY WAYLAND_DISPLAY
# Use SDL's direct native EGL window path for this full-panel port.
# The H700 SDL default uses three rotating pixmaps plus an asynchronous blitter.
# GTA's Android wrapper changes rendering threads and performs partial redraws;
# this trial bypasses the pixmap path without altering the game or its effects.
export SDL_BLITTER_DISABLED=1
unset SLOT_GTA_OPAQUE SLOT_GTA_POSTFX_COMPAT
# The private Android wrapper replaces ALSOFT_CONF at startup. Pin only its
# audio config so the existing Slot volume rocker continues to control output.
export SLOT_GTA_ALSOFT_CONF="${SDCARD_PATH}/Config/ports-alsoft.conf"
export ALSOFT_CONF="$SLOT_GTA_ALSOFT_CONF"
export LD_PRELOAD="${SDCARD_PATH}/System/slot-gta-audio.so:${SDCARD_PATH}/System/slot-no-grab.so"
export SDL_AUDIODRIVER=alsa ALSOFT_DRIVERS=alsa
# The general PortMaster library pack includes Mesa EGL. These native GLES
# games must resolve the device's Mali EGL, including ELF-linked imports.
native_gl="${TMPDIR:-/tmp}/slot-native-gles"
mkdir -p "$native_gl"
for lib in libEGL.so.1 libGLESv2.so.2; do
    [ -f "/usr/lib/$lib" ] || exit 1
    ln -sf "/usr/lib/$lib" "$native_gl/$lib"
done
export LD_LIBRARY_PATH="$native_gl${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export SDL_VIDEO_GL_DRIVER=/usr/lib/libGLESv2.so.2 SDL_VIDEO_EGL_DRIVER=/usr/lib/libEGL.so.1
export GTASA_STICK_MAP=0,1,3,4 GTASA_FORCE_AUDIO=1 MALLOC_CHECK_=0
# portwatch provides the GTASA-Gamepad normalized layout and inherited volume.
echo "slot GTA: direct native EGL presentation (SDL_BLITTER_DISABLED=1); original game effects" >> gtasa.log
exec ./gtasa >> gtasa.log 2>&1
