#!/bin/sh
set -u

PSY="${AGS_PSY:-/sys/class/power_supply/axp2202-battery}"
STATE="${AGS_LED_STATE:-/run/slot-led.state}"
INTERVAL="${AGS_LED_INTERVAL:-30}"

LOW="${AGS_LED_LOW:-15}"
CLEAR="${AGS_LED_CLEAR:-20}"

read_or() {
	if [ -r "$1" ]; then
		cat "$1" 2>/dev/null
	else
		echo "$2"
	fi
}

green() {
	echo 1 > "$PSY/work_led" 2>/dev/null
	echo 0 > "$PSY/lowpwr_led" 2>/dev/null
}

red() {
	echo 0 > "$PSY/work_led" 2>/dev/null
	echo 1 > "$PSY/lowpwr_led" 2>/dev/null
}

dark() {
	echo 0 > "$PSY/work_led" 2>/dev/null
	echo 0 > "$PSY/lowpwr_led" 2>/dev/null
}

last() { read_or "$STATE" green; }

apply() {
	case "$1" in
	red) red ;;
	*) green ;;
	esac
	echo "$1" > "$STATE" 2>/dev/null
}

want() {
	was="$1"
	cap="$(read_or "$PSY/capacity" "")"
	status="$(read_or "$PSY/status" "")"
	case "$status" in
	Charging | Full)
		echo green
		return
		;;
	esac
	case "$cap" in
	'' | *[!0-9]*)
		echo "$was"
		return
		;;
	esac
	if [ "$was" = red ]; then
		if [ "$cap" -ge "$CLEAR" ]; then echo green; else echo red; fi
	else
		if [ "$cap" -le "$LOW" ]; then echo red; else echo green; fi
	fi
}

case "${1:-loop}" in
off)
	dark
	;;
once)
	apply "$(want "$(last)")"
	;;
loop)
	while :; do
		apply "$(want "$(last)")"
		sleep "$INTERVAL"
	done
	;;
*)
	echo "usage: led.sh [off|once|loop]" >&2
	exit 2
	;;
esac
