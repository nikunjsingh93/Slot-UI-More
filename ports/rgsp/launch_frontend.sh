#!/bin/sh
SD="${SLOT_ROOT:-/mnt/sdcard}"
SYS="$SD/System"
RUN="${AGS_RUN:-/run}"
export SLOT_ROOT="$SD" SLOT_DEVICE=rgsp
# Request the highest stock clock. No overclock or voltage changes.
for policy in /sys/devices/system/cpu/cpufreq/policy*; do
    [ -w "$policy/scaling_governor" ] && printf 'performance\n' > "$policy/scaling_governor"
done
for gpu in /sys/class/devfreq/*; do
    case "$gpu" in *gpu*|*mali*)
        [ -w "$gpu/governor" ] && printf 'performance\n' > "$gpu/governor" ;;
    esac
done
log() { echo "$(date '+%H:%M:%S') $*" >> "$SD/slot-session.log" 2>/dev/null; }
pid="$(cat "$RUN/slot-led.pid" 2>/dev/null)"
if [ -z "$pid" ] || [ ! -d "/proc/$pid" ]; then
    sh "$SYS/led.sh" once
    sh "$SYS/led.sh" loop >/dev/null 2>&1 &
    echo $! > "$RUN/slot-led.pid"
fi
echo "|/bin/sh $SYS/coresave.sh %p %s %e %t" > /proc/sys/kernel/core_pattern 2>/dev/null
[ -f "$SD/slot.log" ] && mv -f "$SD/slot.log" "$SD/slot.log.1"
log "RG SP 0.3.1: fbdev/Mali frontend"
export SLOT_PORTS_DIR="$SD/ports" SLOT_MENU_CODE=312 SLOT_HP_PATH=/sys/kernel/debug/gpio
PY="$SYS/PortMasterH700/bin/python3"
mkdir -p "$SD/ports" "$SD/Games/PORTS" "$SD/Labels/PORTS" "$SD/Config"
# The bundled Python's dependencies live in the runtime's private library dir.
PORT_LIBS="$SYS/PortMasterH700/lib:/usr/lib/aarch64-linux-gnu:/usr/lib"
refresh_ports() { LD_LIBRARY_PATH="$PORT_LIBS" "$PY" "$SYS/slot-refresh-ports.py" "$SD"; }
sh "$SYS/slot-h700-wifi.sh" "$SD" >"$SD/Config/wifi.log" 2>&1 &
refresh_ports || log 'Port rescan failed; check runtime libraries'
while :; do
    /lib/ld-linux-aarch64.so.1 "$SYS/slot" >"$SD/slot.log" 2>&1
    result=$?
    if [ "$result" -ne 85 ] || [ ! -s "$SD/Config/port.request" ]; then exit "$result"; fi
    port_id=$(cat "$SD/Config/port.request")
    rm -f "$SD/Config/port.request"
    SLOT_PORT_ID="$port_id" SLOT_ALSA_DEVICE="${SLOT_ALSA_DEVICE:-plughw:0,0}" LD_LIBRARY_PATH="$PORT_LIBS" "$SYS/slot-portwatch" /bin/sh "$SYS/slot-h700-port-runner.sh" "$port_id" "$SD" >>"$SD/Config/ports.log" 2>&1 || log 'Port exited with an error'
    refresh_ports || true
    sleep 0.2
done
