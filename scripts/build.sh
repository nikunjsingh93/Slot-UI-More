#!/bin/sh
set -eu
[ "$(uname -m)" = aarch64 ] || { echo 'Use the ARM64 build container.' >&2; exit 1; }
repo=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
task_build=${SLOT_BUILD_DIR:-/tmp/slot-g350-portable}
mkdir -p "$task_build/core-source" "$repo/dist"
python3 "$repo/scripts/prepare_build.py" "$repo" "$task_build"
python3 "$repo/scripts/build_accepted_cores.py" "$repo" "$task_build"
cmake -S "$task_build/core-source/ppsspp" -B "$task_build/ppsspp-build" \
    -DLIBRETRO=ON -DUSING_GLES2=ON -DUSING_QT_UI=OFF -DHEADLESS=OFF \
    -DUNITTEST=OFF -DUSE_DISCORD=OFF -DUSE_SYSTEM_LIBPNG=OFF \
    -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_FLAGS=-mcpu=cortex-a35 -DCMAKE_CXX_FLAGS=-mcpu=cortex-a35
cmake --build "$task_build/ppsspp-build" --target ppsspp_libretro -j4
mkdir -p "$task_build/slot"
cp -a "$repo/slot/." "$task_build/slot/"
cd "$task_build/slot"
cargo build --locked --profile device --bin slot --no-default-features --features device
cp target/device/slot "$task_build/root/usr/bin/slot-g350"
cp "$task_build/accepted-cores/flycast_libretro.so" "$task_build/root/usr/share/slot/"
cp "$task_build/accepted-cores/parallel_n64_libretro.so" "$task_build/root/usr/share/slot/"
cp -a "$task_build/accepted-cores/notices/." "$task_build/root/usr/share/slot/notices/"
cp "$task_build/ppsspp-build/lib/ppsspp_libretro.so" "$task_build/root/usr/share/slot/"
strip --strip-unneeded "$task_build/root/usr/share/slot/flycast_libretro.so" "$task_build/root/usr/share/slot/ppsspp_libretro.so"
mkdir -p "$task_build/root/usr/share/slot/PPSSPP"
cp -a "$task_build/core-source/ppsspp/assets/." "$task_build/root/usr/share/slot/PPSSPP/"
cp "$repo/docs/G350-0.1.10-SHADERS.txt" "$task_build/root/usr/share/slot/SHADERS.txt"
cp "$repo/firmware/slot-start" "$task_build/root/usr/bin/slot-start"
chmod 755 "$task_build/root/usr/bin/slot-start" "$task_build/root/usr/bin/slot-g350"
gcc -O2 -Wall -Wextra "$repo/firmware/ports/portwatch.c" -o "$task_build/slot-portwatch" -lasound -lm
gcc -O2 -Wall -Wextra -shared -fPIC "$repo/firmware/ports/slot-no-grab.c" -o "$task_build/slot-no-grab.so" -ldl
python3 "$repo/firmware/ports/patch_system.py" "$task_build/root" "$task_build/slot/target/device/slot" "$task_build/slot-portwatch" "$task_build/slot-no-grab.so"
# The complete image bundles the pinned upstream manager. Existing installs are preserved.
if [ -n "${SLOT_PORTMASTER_ZIP:-}" ]; then
    cp "$SLOT_PORTMASTER_ZIP" "$task_build/root/usr/share/slot/portmaster.zip"
fi
for task_core in ppsspp; do
    mkdir -p "$task_build/root/usr/share/slot/notices/$task_core"
    for task_notice in LICENSE LICENSE.TXT COPYING README.md; do
        if [ -f "$task_build/core-source/$task_core/$task_notice" ]; then
            cp "$task_build/core-source/$task_core/$task_notice" "$task_build/root/usr/share/slot/notices/$task_core/"
        fi
    done
    cp "$repo/patches/core-sources-v019.json" "$task_build/root/usr/share/slot/notices/$task_core/sources.json"
done
python3 "$repo/scripts/verify_device_entrypoint.py" "$task_build/root/usr/bin/slot-g350"
python3 "$repo/scripts/package.py" "$task_build/root" "$repo/dist"
