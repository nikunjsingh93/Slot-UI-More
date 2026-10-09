#!/bin/sh
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
tarball="${1:?usage: run.sh TARBALL [PATCHDIR]}"
patches="${2:-$here/..}"
cc="${CC:-cc}"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

top="$(tar -tzf "$tarball" | head -1 | cut -d/ -f1)"
tar -xzf "$tarball" -C "$work"
src="$work/$top"
[ -f "$src/serial_proto.c" ] || { echo "$tarball has no $top/serial_proto.c" >&2; exit 1; }

asan="-fsanitize=address"
if ! echo 'int main(void){return 0;}' | "$cc" $asan -x c -o "$work/probe" - 2>/dev/null; then
	echo "note: $cc has no working -fsanitize=address, running on assertions alone"
	asan=""
fi

build() {
	mkdir -p "$1"
	cp "$src/serial_proto.c" "$src/serial.h" "$1/"
	"$cc" -I"$1" -I"$here" $asan -g -O1 -o "$1/awtest" "$here/awtest.c"
}

build "$work/unpatched"
for p in "$patches"/*.patch; do
	git -C "$src" apply -p1 "$p"
done
build "$work/patched"

if "$work/unpatched/awtest" >"$work/unpatched.log" 2>&1; then
	echo "FAIL: unpatched gpSP passed, so this test no longer proves anything" >&2
	exit 1
fi
echo "ok: unpatched gpSP fails, as it must"
grep -m1 -E 'ERROR|SUMMARY|Assertion|assert' "$work/unpatched.log" || true

if ! "$work/patched/awtest"; then
	echo "FAIL: patched gpSP still overflows the peer queue" >&2
	exit 1
fi
echo "ok: patched gpSP holds the queue-length invariant across the session flip"
