#!/bin/sh
# Optional credentials are data for wpa_supplicant, never shell commands.
set -eu
SD=$1
cfg="$SD/Config/wifi.conf"
[ -s "$cfg" ] || exit 0
RUN=${SLOT_NET_RUN:-/run}
SYS=${SLOT_NET_SYS:-/sys/class/net}
mkdir "$RUN/slot-wifi-lock" 2>/dev/null || exit 0
trap 'rmdir "$RUN/slot-wifi-lock" 2>/dev/null || true' EXIT
n=0
while [ ! -d "$SYS/wlan0" ]; do
    n=$((n+1));[ "$n" -le 25 ] || { echo 'slot wifi: wlan0 did not appear';exit 1; }
    sleep 1
done
command -v rfkill >/dev/null 2>&1 && rfkill unblock wifi || true
ip link set wlan0 up
if ! pidof wpa_supplicant >/dev/null 2>&1; then
    mkdir -p "$RUN/wpa_supplicant"
    wpa_supplicant -B -i wlan0 -c "$cfg" -P "$RUN/slot-wpa.pid"
fi
if ! ip -4 addr show dev wlan0 | grep -q 'inet '; then
    udhcpc -i wlan0 -p "$RUN/slot-udhcpc.pid" -t 5 -T 3 -b -s /usr/share/udhcpc/default.script
fi
echo 'slot wifi: connection/DHCP requested; status is available through wpa_cli'
