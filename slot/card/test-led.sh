#!/bin/sh
set -eu

HERE="$(cd "$(dirname "$0")" && pwd)"
SCRIPT="$HERE/System/led.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT HUP INT TERM

PSY="$TMP/psy"
mkdir -p "$PSY"
STATE="$TMP/state"

gauge() {
	printf '%s\n' "$1" > "$PSY/capacity"
	printf '%s\n' "$2" > "$PSY/status"
}

led() {
	AGS_PSY="$PSY" AGS_LED_STATE="$STATE" sh "$SCRIPT" "$@"
}

lights() {
	printf '%s/%s' "$(tr -d ' \n' < "$PSY/work_led")" "$(tr -d ' \n' < "$PSY/lowpwr_led")"
}

expect() {
	got="$(lights)"
	if [ "$got" != "$1" ]; then
		echo "$3: expected green/red = $1, got $got" >&2
		exit 1
	fi
}

gauge 80 Discharging
led once
expect "1/0" "" "healthy battery"

gauge 15 Discharging
led once
expect "0/1" "" "at the low threshold"

gauge 17 Discharging
led once
expect "0/1" "" "inside the hysteresis band, coming from red"

gauge 20 Discharging
led once
expect "1/0" "" "at the clear threshold"

gauge 16 Discharging
led once
expect "1/0" "" "just above the low threshold, coming from green"

gauge 3 Charging
led once
expect "1/0" "" "charging on an almost flat battery"

gauge 3 Full
led once
expect "1/0" "" "full"

gauge 5 Discharging
led once
expect "0/1" "" "flat and discharging"
remembered="$(cat "$STATE")"
led off
expect "0/0" "" "off"
if [ "$(cat "$STATE")" != "$remembered" ]; then
	echo "off forgot the remembered colour: $(cat "$STATE") != $remembered" >&2
	exit 1
fi
led once
expect "0/1" "" "restored after off"

gauge 80 Discharging
led once
expect "1/0" "" "healthy, before the gauge breaks"
printf 'not-a-number\n' > "$PSY/capacity"
led once
expect "1/0" "" "unparseable gauge holds the last colour"
rm -f "$PSY/capacity"
led once
expect "1/0" "" "absent gauge holds the last colour"

gauge 80 Discharging
led once
expect "1/0" "" "healthy, before interference"
printf '0\n' > "$PSY/work_led"
printf '0\n' > "$PSY/lowpwr_led"
led once
expect "1/0" "" "the light is re-asserted after being darkened externally"

if led bogus 2>/dev/null; then
	echo "an unknown subcommand exited 0" >&2
	exit 1
fi

echo "led: all passed"
