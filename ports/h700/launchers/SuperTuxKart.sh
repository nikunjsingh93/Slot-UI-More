#!/bin/sh
GAMEDIR="${SDCARD_PATH}/ports/.ports/supertuxkart"
cd "$GAMEDIR" || exit 1
. "$SDCARD_PATH/System/slot-h700-native-gl.sh"
export PORTMASTER_HOME="$GAMEDIR"
export SUPERTUXKART_DATADIR="$GAMEDIR" SUPERTUXKART_ASSETS_DIR="$GAMEDIR/data/"
export LD_LIBRARY_PATH="$GAMEDIR/libs:$SDCARD_PATH/System/PortCompat/supertuxkart/lib:$LD_LIBRARY_PATH"
screen=480x320
[ "${SLOT_DEVICE:-}" != rg35xxsp ] || screen=480x360
# Mobile assets and the GLES build; scale a 3:2 picture, without stretching it.
exec ./supertuxkart --fullscreen --screensize="$screen" --render-driver=gl --disable-dynamic-lights --disable-ssao --disable-ibl --disable-dof --disable-motion-blur --disable-mlaa --disable-hd-textures --shadows=0 --anisotropic=0 >> log.txt 2>&1
