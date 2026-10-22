# Project instructions for coding agents

These instructions apply to this repository. Codex, Claude Code, OpenCode and
Pi Agent should use the same project notes; no external memory service is needed.

## Start a session

1. Read `.ai/PROJECT.md` for architecture, accepted decisions and pitfalls.
2. Read `.ai/HANDOFF.md` for the latest verified state and unfinished work.
3. Check `git status --short`, recent commits and the relevant source/tests.
   Documentation can be stale; resolve conflicts against evidence and the user's
   latest instructions. Distinguish observed facts from assumptions.
4. Resume the confirmed baseline instead of recreating past experiments.

`CLAUDE.md` points to these same files. If an agent does not automatically load
project instructions, explicitly open these three Markdown files at startup.

## During development

- Preserve unrelated changes and existing instructions. Do not reset, clean or
  delete untracked files merely because they are untracked. This checkout has
  historical rejected experiments outside the published baseline.
- Prefer focused changes to rewrites. Identify the target model before editing
  frontend, boot, graphics, input or packaging code.
- G350 and H700 are separate Rust workspaces/backends. Use **different Cargo
  target directories**; do not interchange their binaries or boot files.
- Preserve the owner-accepted Dreamcast/N64 cores and settings unless a task
  explicitly changes them. Consult manifests before choosing a binary.
- Preserve long-press Menu-alone exit. On H700, preserve per-game Menu+Select
  D-pad/left-stick mode and Start selecting the N64 backend on the shelf.
- Do not silently stretch native NFS U2, restore rejected Wave Race VBO tuning,
  or enable the abandoned GTA graphics interposers. See PROJECT.md.
- Do not add proprietary games, BIOS, APK/OBBs, extracted Android libraries,
  generated commercial-game code/executables, saves, credentials or commercial
  artwork to Git or public releases. Keep source/notice obligations intact.
- Never assume the connected SD card's drive letter or model. Inspect it before
  writing. Preserve games, labels, BIOS, saves and settings during an update.
  Check exact resolved targets before recursive Windows deletions; use literal
  paths and one shell throughout. Do not flash a card during a source-only task.
- Keep durable findings and failed approaches in PROJECT.md, with links to
  evidence. Put temporary progress and concrete next actions in HANDOFF.md.
- Do not infer permission to publish, message others or delete data from these
  notes; follow the current user's authorization and the host's tool policies.

## Verification and releases

- Run checks appropriate to the changed component. Record exact commands,
  results and any skipped tests. Documentation-only edits need document/path
  checks, not a firmware rebuild.
- A container build, simulated input test or filesystem check is **not** a
  physical boot, gameplay, audio, suspend or performance test. State the limit.
- Inspect source staging for private files; stage explicit intended paths rather
  than sweeping historical untracked artifacts into a release.
- When a release is requested, use one synchronized version across the three
  models and one complete image per model, with consistent release titles.
  Publish large images as GitHub release assets, not Git blobs.
- Preserve model boot regions and accepted cores, validate the embedded installer
  and filesystems, verify archive/image hashes, and compare uploaded asset
  sizes/SHA-256 before calling a release complete. Retain corresponding sources.

## Checkpoint and hand off

After every substantial completed task, before handing off or switching agents,
and before ending a development session:

1. Update `.ai/HANDOFF.md`: date, completed work, changed files, actual Git state,
   outstanding issues, commands/results and the next specific step.
2. Update `.ai/PROJECT.md` only for lasting architecture, decisions or discoveries.
3. Separate newly run checks from historical results and owner reports. Do not
   claim a push, card installation, hardware test or fix without evidence.
4. Keep notes concise, public-safe and repository-relative; link detailed records
   instead of copying logs or entire conversations. Do not erase valid history.

Checkpoints must be written while the session is active; a crash or quota limit
may prevent a final update. These files provide continuity, not automatic access
to every agent's conversation history.
