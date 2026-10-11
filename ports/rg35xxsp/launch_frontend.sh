#!/bin/sh
SD="${SLOT_ROOT:-/mnt/sdcard}"
SYS="$SD/System"
RUN="${AGS_RUN:-/run}"
export SLOT_ROOT="$SD" SLOT_DEVICE=rg35xxsp
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
log "RG35XX SP 0.2.0: fbdev/Mali frontend"
exec /lib/ld-linux-aarch64.so.1 "$SYS/slot" >"$SD/slot.log" 2>&1
