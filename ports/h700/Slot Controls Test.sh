#!/bin/sh
GAMEDIR="${SDCARD_PATH}/ports/.ports/slot-controls-test"
cd "$GAMEDIR" || exit 1
exec ./slot-controls-test
