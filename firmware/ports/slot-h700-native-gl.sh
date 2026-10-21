#!/bin/sh
# Slot native GLES / canonical controller setup, only for the added SDL ports.
unset SDL_VIDEODRIVER DISPLAY WAYLAND_DISPLAY SDL_GAMECONTROLLERCONFIG_FILE
export LD_PRELOAD="${SDCARD_PATH}/System/slot-no-grab.so"
export SDL_AUDIODRIVER=alsa ALSOFT_DRIVERS=alsa
native_gl="${TMPDIR:-/tmp}/slot-native-gles"
mkdir -p "$native_gl"
for lib in libEGL.so.1 libGLESv2.so.2; do
    [ -f "/usr/lib/$lib" ] || exit 1
    ln -sf "/usr/lib/$lib" "$native_gl/$lib"
done
export LD_LIBRARY_PATH="$native_gl:$SDCARD_PATH/System/PortMasterH700/system-lib:$SDCARD_PATH/System/PortMasterH700/lib:/usr/lib/aarch64-linux-gnu:/usr/lib"
export SDL_VIDEO_GL_DRIVER=/usr/lib/libGLESv2.so.2 SDL_VIDEO_EGL_DRIVER=/usr/lib/libEGL.so.1
export SDL_GAMECONTROLLERCONFIG='190000004b4800000011000000010000,GO-Super Gamepad,a:b1,b:b0,x:b2,y:b3,leftshoulder:b4,rightshoulder:b5,lefttrigger:b6,righttrigger:b7,back:b12,start:b13,guide:b16,leftstick:b14,rightstick:b15,dpup:b8,dpdown:b9,dpleft:b10,dpright:b11,leftx:a0,lefty:a1,rightx:a2,righty:a3,platform:Linux,'
