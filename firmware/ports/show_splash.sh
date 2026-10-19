#!/bin/sh
# The G350 build uses one Slot image for boot/return, without legacy overrides.
[ -w /dev/fb0 ] || exit 0
exec /usr/bin/magick /usr/share/slot/splash.png -depth 8 bgra:/dev/fb0
