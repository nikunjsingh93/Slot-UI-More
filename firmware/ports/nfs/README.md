# Private matching NFS build

The shipped guest object archive does not match the advertised executable.
These scripts disable accelerations tied to that other layout, and compile a
fresh lift instead. The generic GLES/HLE backend and controller mapping remain.
Only the Cyrix probe and CRT memmove overrides were verified for this input.
This is a compatibility build; the original native acceleration performance
must not be assumed. Hardware testing remains necessary.

Keep the entire build outside this repository. **Never commit the input EXE,
generated C, objects or resulting game-linked executable.** The output is private
and is not part of the public firmware/source ZIP.

Pinned inputs:

* `manuelpllull/nfsu2-recompiled-r36s` commit
  `6d45c4ea9b8a4a3a6cab97f9af9f427eb3486d38`, recursive submodules.
* `sp00nznet/pcrecomp` base `35548b8`, then the 14 patches in the NFS source's
  `patches/pcrecomp` directory, in filename order.
* Owner's EXE SHA256:
  `f9dd86c054878ce6276beb07c1fd61874f7a1e4bf1f241b084c65b73e24168a7`.

Use an ARM64 Linux builder with GCC/G++ 10 or newer, Python, pefile, capstone,
binutils and standard build tools. The successful local build used Debian
Bullseye ARM64. SDL/GLES headers come from the pinned bundled SDK. Allow roughly
10 GB of temporary space and several GB of RAM; compilation uses eight workers.

1. Clone both projects into sibling directories named `nfs-source` and
   `pcrecomp`; initialize submodules and apply the patches above.
2. In `nfs-source/config`, clear `hooks.txt` and `exclude.txt`. Keep only the
   existing two verified addresses in `overrides.txt` (006F5FC5, 0075CFD0).
   Run `python prepare-runtime.py /absolute/path/nfs-source`.
3. Run the upstream tools with private paths: `scripts/normalize_exe.py EXE
   analysis.exe`, then pcrecomp's `tools/cpp/vtable_scan.py analysis.exe --seeds
   seeds.json`, then `scripts/run_lift.py catalog analysis.exe seeds.json
   catalog.json`, then `scripts/run_lift.py lift analysis.exe catalog.json
   src/recomp/gen`. `PCRECOMP` can specify the absolute pcrecomp directory.
4. For Python 3.9, remove the `newline="\n"` keyword from the `write_text` call
   in upstream `scripts/split_android_sources.py`; Python 3.10+ accepts it.
5. Run `python compile-guest.py /absolute/path/nfs-source /private/build`.
   This uses the NFS bundled recomp32 SDK, not pcrecomp's older runtime headers.
   It deliberately omits `NFS_X87_LW`, whose per-function tables are for the
   other executable. Large units use O1 to limit compiler resources.
6. In `nfs-source`, run `ports/r36s/build_ci.sh` with `CC=gcc CXX=g++ STRIP=strip
   OBJDUMP=objdump`, `BUILD_DIR=/private/build/host`,
   `GUEST_DIR=/private/build/guest` and `OUT=/private/build/output`.
   The preparation script adds the POSIX timer library needed on Bullseye.
7. Test with the untouched original game directory and a real GLES driver.
   A virtual X11 display with Mesa software GLES can check startup but cannot
   measure RK3326 performance or verify handheld sound/buttons. The local test
   used `NFSU2_PARALLEL_GUEST=0`, `--cap30 --lang=English`.

Install only `output/nfsu2.aarch64` into the owner's existing `ports/nfsu2` after
testing. Do not copy test-created profiles or overwrite game data and saves.
