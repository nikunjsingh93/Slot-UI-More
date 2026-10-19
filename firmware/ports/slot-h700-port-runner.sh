#!/bin/sh
set -eu
id=$1
SD=$2
export SDCARD_PATH="$SD" PLATFORM=h700
export DEVICE="$SLOT_DEVICE" RGXX_MODEL="$SLOT_DEVICE"
export USERDATA_PATH="$SD/Config/slot-ports" SHARED_USERDATA_PATH="$SD/Config/slot-ports"
export LOGS_PATH="$SD/Config/slot-ports/logs"
mkdir -p "$LOGS_PATH" "$USERDATA_PATH/PORTS-portmaster" "$SD/ports"
export SLOT_MENU_CODE=312
export LD_PRELOAD="$SD/System/slot-no-grab.so${LD_PRELOAD:+:$LD_PRELOAD}"
export AUDIODEV="${SLOT_PORT_PCM:-plughw:0,0}"
# Use the same software gain for OpenAL ports as SDL/ALSA ports.
export ALSOFT_CONF="$SD/Config/ports-alsoft.conf"
printf '[alsa]\ndevice = %s\n' "$AUDIODEV" > "$ALSOFT_CONF"
case "$id" in
portmaster)
    file="$SD/ports/PortMaster.sh"
    printf '# Portmaster.sh\n# Slot manager entry\n' > "$file"
    ;;
script:*)
    name=${id#script:}
    case "$name" in ''|.*|*[!a-zA-Z0-9\ ._\(\)-]*|*/*) exit 2;; esac
    case "$name" in *.sh) ;; *) exit 2;; esac
    file="$SD/ports/$name"
    ;;
*) exit 2;;
esac
[ -f "$file" ] || exit 2
exec /bin/sh "$SD/System/PortMasterH700/launch.sh" "$file"
