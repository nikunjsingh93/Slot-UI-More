# Development handoff

Last updated: **2026-10-22, America/New_York**. Agent: Codex.

## Publication follow-up completed

User explicitly requested pushing this update to all three firmwares. Complete
1.1.17 images are published under g350-v1.1.17, rgsp-v1.1.17 and
rg35xxsp-v1.1.17, with consistent Complete Image titles and G350 marked Latest.
All 19 uploaded assets match the local exact size/SHA-256 records. Source/tag
commit is `23c04f7883df86b146e890c2bc1def34f0f32530`. Corresponding native N64,
four added cores and compatibility-library sources are included/retained.
No card was written. Old releases/source obligations were preserved.

All three images passed local composition and readbacks, retaining the accepted
cores and model boot files. G350 was rebuilt on native Linux after a mapped-
Windows mtools image failed readback; the failed image was replaced, not shipped.
Both final SP rootfs regions match their checked files exactly. Actual BusyBox
installers check all 15 system folders, 13 cores, 189 PSP assets, installed Studio,
idempotence, corrupted payload rejection and interrupted retry/game preservation.
The G350 provisioning fragment passes fresh/repeat/Studio refresh and custom-core/
game-preservation checks. Physical first boot, new-system gameplay, audio quality
and performance remain unverified; the 46 software tests are not hardware tests.

Release files: enclosing workspace `outputs/release-1.1.17`; native staging
`/tmp/slot-release-117`. Build/composition records: `scripts/release-1.1.17`.
Exact public records: `docs/releases/assets-1.1.17.json`, `uploads-1.1.17.json`,
`validation-1.1.17.json` and per-model manifests. This metadata follow-up updates
README/download records on main. Pages deployment verification is the remaining
publication check at this checkpoint; the push includes Studio changes.

Next physical check, when the owner is ready: fresh boot and one game from each
added system; verify arcade BIOS/set matching, audio, states, Menu quit, labels
and H700 per-system screen settings. Do not assume a card is connected. Earlier
notes below describe the preceding local-test task and are historical.

## New active task: additional console/arcade systems

User requested NES, Arcade, Sega and other common systems in Slot and Artwork
Studio. Both frontends now have six appended platform IDs: NES, MD (Genesis),
SMS, GG, PCE and ARCADE. Existing IDs remain stable. Four actual ARM64 cores
were built from pinned official sources: FCEUmm, Genesis Plus GX, Beetle PCE Fast
and FBNeo. The accepted nine existing G350 core hashes are preserved.
The next-build app version is synchronized to **1.1.17**; public images remain
1.1.16. Nothing from this task has been committed, pushed, flashed or released.
Historical untracked prototypes remain unrelated; do not bulk-add them.

New runtime behavior includes console save/load chords, full console video,
Game Gear aspect preservation, H700 per-system screen entries, original stylized
cartridge/PCB shells, FBNeo friendly names/BIOS filtering and portrait rotation.
Do not add the digital cores to H700's 3D-only analog conversion/toggle clauses;
they must keep Select/coin and digital D-pad input. The newly added regression
tests cover this distinction. Existing single-cart refusal-to-eject behavior is
unchanged; tests use a multi-game shelf like the accepted control tests.

Studio supports fourteen emulated systems. NES/MD/SMS/GG/PCE use physical scans
with logo/cover fallback; Arcade uses logos/covers and pinned FBNeo short-name
aliases. Existing label protection and no-ROM-read behavior are retained.
Original eight-system catalogs compare exactly to HEAD. Source pins are in
`docs/additional-core-sources.json`; guide is `docs/ADDITIONAL-SYSTEMS.md`.

Software checks so far: 46 targeted Rust tests pass across both workspaces
(scanner/platform, cartridge geometry, arcade rotation, controls, H700 menus).
Real NES/MD/SMS/GG/PCE cores passed original test-ROM video/PCM/state roundtrips;
FBNeo handled an invalid archive without crashing. Six-system browser tests pass
for matching, aliases, BIOS filtering, PNG dimensions, ZIP/direct save, existing
labels, nested folders and mobile layout. Live artwork downloads succeeded for
all six. G350 optimized entrypoint selects KMSDRM. No actual arcade ROM/gameplay,
handheld boot, audio quality or performance check has been performed.

Build cache: ARM64 Docker `slot-g350-compiler`; `/tmp/slot-extra` holds pinned
sources/cores/logs and staging. G350 target `/tmp/slot-release-116/g350-target`;
H700 target `/tmp/slot-sp-036/target`. Test outputs are under the enclosing
workspace's `outputs/extra-systems-1.1.17`. `scripts/build_extra_cores.py` and
`scripts/package_extra_systems_test.py` are portable entry points with explicit
paths. Packages are system-file overlays for existing Slot cards, NOT full flash
images. G350 SYSTEM is read back and checked against all new/accepted core hashes.

Final optimized binaries and package payload hashes have been checked. H700
frontend SHA-256 is `048dc4a7a7052233e191469eef056fb24b1a37210ca8a8e5503f592d9bfd6a25`
in both SP ZIPs; G350 is `81f01d3df8effd12bf461d08fd1ffdcf1ce9b6f29262a4dd178cc004d77f2817`.
`docs/ADDITIONAL-SYSTEMS-VALIDATION.json` records the 46 targeted Rust tests,
real-core smoke tests, six live artwork requests and physical-test limits.
Outer packages retain explicit licenses (including FCEUmm's case-sensitive
`Copying` file), with separate complete pinned core-source archives. A clean
frontend/source snapshot and SHA256SUMS accompany the local test downloads.
No commit/push/card installation or public image refresh was performed.

Next, the owner can install the model's test system files onto an existing Slot
card using START-HERE.txt, then test NES, Sega/PCE and compatible arcade games,
audio, Menu exit, states, screen settings and labels. For future publication,
compose clean full images for all three
models, add these cores/notices/sources and update system manifests/installer
checks; do not imply these test overlays already replaced public image downloads.

## Current task and result

The user asked for persistent cross-agent memory using AGENTS.md, CLAUDE.md,
`.ai/PROJECT.md` and this file, populated from the conversation and actual code.
These four files were created in the Git repository. No existing versions were
found. No application code, firmware image, SD card, release or agent settings
were changed for this documentation setup. No memory service was installed.

The user subsequently requested committing and pushing these four files on
2026-10-22. This handoff accompanies that documentation-only commit. Its scope
is AGENTS.md, CLAUDE.md, PROJECT.md and HANDOFF.md; historical experiments remain
outside the commit. Check the latest Git log and origin/main for delivery state.
No firmware rebuild, image replacement or release publication is part of this
follow-up.

## Verified repository state at session start

- Git root is the project folder containing this file's parent `.ai/` directory;
  in the current enclosing development workspace it is `work/github-g350`.
- Origin is `https://github.com/nikunjsingh93/Slot-UI-More.git`.
- HEAD: `50daa3c` — record 1.1.16 source/image download checksums.
- Earlier release commits: `11d40ab` (runtime/source/docs transfer), `7811490`
  (composition/source scripts). Release source ZIPs record the full `7811490`
  commit; later checksum metadata did not change runtime code.
- No tracked application changes were present. Historical untracked prototypes
  were present before this task: GTA opaque/postfx helpers and tests, a Most
  Wanted launcher/diagnostic records, private-build patches, and RG SP 0.3.x
  trial patches/notes including rejected VBO files. Leave them alone; they are
  not newly unfinished implementations or approved release inputs.

## Most recent completed development

Slot **1.1.16** complete images were published on 2026-10-21 for:

- [G350](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/g350-v1.1.16)
- [RG SP](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rgsp-v1.1.16)
- [RG35XX SP](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/rg35xxsp-v1.1.16)

The preceding session verified all 19 uploaded assets by GitHub size/SHA-256,
published all three pages and checked G350 was marked Latest. That is a recorded
prior-session result; no remote release query was rerun for this memory setup.

The H700 transfer includes native Rice/per-game ParaLLEl on Start, faster exit
and shelf restoration, accepted Dreamcast fixes, PSP width, per-system screen
size, native-port mappings and example launchers. G350 preserves its accepted
cores and adds synchronized version/state-layout protection. Studio v5 installs
on H700. Commercial games/BIOS/private game-linked engines are excluded.
Most Wanted Mobile was removed; GTA III, Vice City, RVGL and SuperTuxKart had
owner-confirmed RG SP success. A Reddit post was drafted, not posted by an agent.

## Important current files

- `docs/RELEASE-1.1.16.md`, `docs/releases/validation-1.1.16.json`, manifests and
  `assets-1.1.16.json`: release contents, source commit, hashes and limits.
- `slot/` vs `ports/h700/frontend/`: the separate current frontend workspaces.
- `firmware/ports/port-input.h`, native host/runner/audio helper: accepted controls
  and native integration. Model launchers live under each model's `ports/` folder.
- `ports/h700/README.md`, `ADDITIONAL-PORTS.md`, `n64-native/`: setup/build context.
- `scripts/build_latest_images.py`, `scripts/release-1.1.16/`: image composition.
- `studio/`: hosted/offline artwork app; current source in `studio/source/`.
- This task adds only `AGENTS.md`, `CLAUDE.md`, `.ai/PROJECT.md`, `.ai/HANDOFF.md`.

## Verification record

**Historical 1.1.16 results, not rerun today:**

- 28 H700 UI/panel/selection tests, 22 screen-menu tests, 2 optimized H700 audio
  tests and 11 G350 production-profile tests passed: 63 Rust tests in total.
- Supervisor input/volume/Menu tests, both model native N64 config tests,
  launcher syntax and actual OpenAL routing checks passed.
- Both model BusyBox installer tests, retry/idempotence/corruption checks,
  filesystem/boot-region/readback and image/ZIP hash checks passed.
- An unoptimized audio timing run failed under the build environment; the
  production-profile test passed without changing firmware timing. Do not
  report the debug timing failure as a hardware regression or conceal it.

**Commands/checks in this documentation session:** read the attached request;
searched for existing instruction/memory files; inspected `git status --short`,
`git log -5 --oneline`, `git remote -v`, source/build files and release records;
checked `git check-ignore` for the four target paths (none ignored). After
writing, a Python standard-library check passed for all four files' UTF-8,
local Markdown links, CLAUDE.md import targets and key evidence paths. A final
`git status --porcelain --untracked-files=no` was empty, confirming tracked
application files remain unchanged. Status for the requested paths showed
only the four new documentation files. No compiler, firmware rebuild or
hardware test was run for this documentation-only setup.

## Open issues and next steps

There is **no implementation task currently in progress**. The concrete
outstanding release validation is:

1. When the user is ready, ask for first-boot results from the published full
   images. Prioritize RG35XX SP: its transferred changes have software checks
   but no documented physical test. Do not assume a card is currently connected.
2. If testing proceeds, verify the model/image version and back up relevant
   files; check N64 Rice/ParaLLEl selection and Menu return, Dreamcast audio,
   PSP proportions, screen settings, volume and native controls. Collect logs
   from Slot/Config and affected game directories before proposing fixes.
3. GTA SA's last audio-config guard still lacks explicit owner confirmation
   of its physical volume rocker. Native-port sleep/lid behavior is unverified.
4. Keep demanding N64/game-performance limits honest. Do not restart the
   removed Most Wanted effort or rejected Wave Race/GTA graphics experiments
   without a new user request and evidence.

If switching agents now, read PROJECT.md, inspect the actual Git state, then
wait for the user's next requested change or validation results. Do not reflash,
republish or add speculative optimizations merely to clear these pending tests.

## Updating these notes

Record the next substantial task's exact changes, commands/results, user test
feedback and remaining step here. Move lasting decisions into PROJECT.md and
link detailed records. Keep unpublished local prototypes distinct from approved
source. Do not store credentials, commercial data or private diagnostic dumps.
