# 1.1.16 composition record

These are the exact staging/composition scripts used in the ARM64 Bullseye
builder for the release. They operate on image files, never a physical card.
Their `/work` and `/tmp/slot-release-116` paths describe the build cache, not
a portable one-command installer. Adjust them to your checked artifact paths.

Current complete frontend sources are `slot/` for G350 and
`ports/h700/frontend/` for both H700 models. Build them with separate Cargo
target directories:

```sh
cargo build --locked --profile device --bin slot --no-default-features --features device
```

Compile `firmware/ports/portwatch.c` with `-O2 -Wall -Wextra -Werror -lasound -lm`.
Compile `slot-no-grab.c` and `slot-gta-audio.c` with `-shared -fPIC -O2 -ldl`.
For the H700 base manager use `ports/h700/prepare_runtime.py` and its pinned
archives. Native N64 has its own `build-native.py`, pinned source archives in
the source ZIP, and the exact muOS reference configuration archived beside them.
The corresponding sources for all 25 SuperTuxKart compatibility libraries are
in the companion compatibility-source ZIP; `stage_stk_compat.py` checks their
binary provenance and retains package notices.

`stage-artifacts.py` merged the previously clean 1.1.15 runtime and verified
accepted card-test deltas, then installed the new frontend/helpers. Those
named historical working directories are local inputs, not additional source
dependencies: the resulting current H700 System payload is embedded in the
release image's `/usr/share/slot-sp/payload.tar.gz`. For a later frontend-only
rebuild, extract that clean payload, replace its frontend/helpers from source,
update its manifest hashes, and compose against the matching official BaseOS
1.3.0 image using `scripts/build_latest_images.py`. Do not use a personal game
card as the packaging baseline.

`repack-g350.py` started with the verified 1.1.15 SYSTEM/KERNEL package, replaced
only the current frontend/version, and asserted all accepted emulator hashes.
Its matching original kernel pair and bootloader were retained. The generic
image composer verified filesystem integrity, payload readback, partition
geometry/boot-region preservation and archive hashes for all three models.

The checked old-image hashes, new image hashes and test counts are in
`docs/releases/`; unchanged core source pins/build recipes remain under
`patches/`, `scripts/` and the retained 1.1.15 source assets. BaseOS archive
hashes are in each model manifest. `package-sources.py` archived the release
Git commit, five native N64 source archives and twenty exact Debian source
packages, and generated per-model checksums.
