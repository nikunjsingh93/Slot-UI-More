# 1.1.17 composition record

Build both frontend workspaces separately as described in `.ai/PROJECT.md`.
`scripts/build_extra_cores.py` builds the four additional cores from exact pins.
`stage.py` stages the previously clean 1.1.16 overlays and new outputs, verifies
the nine accepted G350 core hashes, adds the complete notices/FBNeo DAT and
rebuilds G350 SYSTEM with the expanded offline Artwork Studio.

The paths in these scripts identify this release's checked build caches. Supply
equivalent verified inputs elsewhere; do not use an owner's personal card.
The existing `scripts/build_latest_images.py` composes each model from the staged
inputs and its verified original base. Use a fresh work directory. It checks
model identity, boot-region preservation, filesystems, exact readbacks and real
H700 BusyBox clean/repeated/corrupt/interrupted installation. The installer
creates all fifteen system shelves/folders and 189 free PPSSPP assets.

Use `/tmp` or another native Linux filesystem for image composition. During this
release a G350 image modified through mtools on the Windows-mapped output folder
failed SYSTEM readback. Rebuilding on native storage passed; the failed image
was not published. Final SP embedded-rootfs regions were also compared exactly
against their checked rootfs files. Copy verified final ZIPs out sequentially
and verify destination hashes. The composer now keeps working images in its
work directory and checks the final embedded H700 rootfs hash as well.

After committing the source, `package.py` archives that exact Git commit, the
five native N64 source archives, all four new complete core sources and the
unchanged compatibility-library sources. It records manifests and checksums.
Large images/source archives belong in release assets. No private game files,
BIOS, game-linked engines, saves or downloaded commercial artwork are inputs.
Physical first boot/gameplay/performance testing is separate and remains pending.
