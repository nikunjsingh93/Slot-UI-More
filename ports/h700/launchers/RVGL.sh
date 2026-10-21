#!/bin/sh
GAMEDIR="${SDCARD_PATH}/ports/.ports/rvgl"
cd "$GAMEDIR" || exit 1
. "$SDCARD_PATH/System/slot-h700-native-gl.sh"
export LD_LIBRARY_PATH="$GAMEDIR/lib/libarm64:$LD_LIBRARY_PATH"
exec ./rvgl.arm64 >> log.txt 2>&1
