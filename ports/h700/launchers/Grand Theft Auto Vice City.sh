#!/bin/sh
GAMEDIR="${SDCARD_PATH}/ports/.ports/gtavc"
cd "$GAMEDIR" || exit 1
. "$SDCARD_PATH/System/slot-h700-native-gl.sh"
# Same direct SDL presentation path accepted for GTA SA; native GLES engine.
export SDL_BLITTER_DISABLED=1
export SDL_GAMECONTROLLERCONFIG_FILE="$GAMEDIR/gamecontrollerdb.txt"
exec ./reVC >> log.txt 2>&1
