# Development handoff

Last updated: **2026-10-22, America/New_York**. Agent: Codex.

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
