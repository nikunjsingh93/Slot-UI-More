#!/bin/sh
# User supplies their own verified GTA SA runtime and Android 2.10 data.
GAMEDIR="${SDCARD_PATH}/ports/.ports/gtasa"
cd "$GAMEDIR" || exit 1
# H700 uses vendor fbdev/Mali SDL; KMSDRM/Wayland overrides are incompatible.
unset SDL_VIDEODRIVER DISPLAY WAYLAND_DISPLAY
export SDL_AUDIODRIVER=alsa ALSOFT_DRIVERS=alsa
export SDL_VIDEO_GL_DRIVER=libGLESv2.so.2 SDL_VIDEO_EGL_DRIVER=libEGL.so.1
export GTASA_STICK_MAP=0,1,3,4 GTASA_FORCE_AUDIO=1 MALLOC_CHECK_=0
# portwatch provides the GTASA-Gamepad normalized layout and inherited volume.
exec ./gtasa >> gtasa.log 2>&1
