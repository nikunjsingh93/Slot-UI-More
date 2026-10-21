# Sharing source and firmware

This is an independent GPL-3.0 Slot adaptation. Preserve the root and upstream licenses, attribution and change history. PortMaster GUI is MIT-licensed; its helpers and each port retain their own licenses. Emulator and OS licenses vary; some components may impose additional redistribution/noncommercial terms. Nothing here grants rights to commercial games, BIOS, artwork or trademarks.

## What this release contains

Release 1.1.16 provides complete model-specific images and current full
frontend/build sources. The unchanged 1.1.15 core source archives stay available.
The added H700 native N64 source archives and exact Debian source packages for
the SuperTuxKart compatibility libraries are supplied with 1.1.16, together
with component notices. Private NFS/GTA game-linked engines and commercial game
data/artwork remain excluded. Historical notes below describe earlier packages.

The 0.1.13 sharing ZIP contains SYSTEM, SYSTEM.md5, KERNEL and KERNEL.md5 only. It does not include the owner's game folders, disc scans, GTA APK/OBBs/libraries, generated NFS guest source, linked NFS executable, profiles or saves. The personal artwork update and historical private runtime bundle are not uploaded. PortMaster itself is downloaded from its upstream release; this repository contains integration code and original generic labels/splash.

## Source obligations

Distributing GPL binaries requires the applicable complete corresponding source and build/install material. Publish it with comparable access to the binaries, retain notices and satisfy the precise license version. General project links alone may not fulfill this duty. See [GNU's FAQ](https://www.gnu.org/licenses/gpl-faq.html#SourceAndBinaryOnDifferentSites) and [GPLv3 section 6](https://www.gnu.org/licenses/gpl-3.0.html#section6).

For these binaries, retain the release source snapshot, repository history/patches, and the corresponding core source archives in [the 1.1.15 release](https://github.com/nikunjsingh93/Slot-UI-More/releases/tag/g350-v1.1.15) and the source/restore records under docs/ and patches/. The unchanged base OS/kernel provenance is recorded in docs/SOURCE-REVISIONS.txt and the historical source archives. Keep source accessible for as long as the relevant distribution obligations require. Do not delete historical source assets while distributing binaries that depend on them.

Before making all historical firmware releases public, audit component-by-component source coverage, notices and redistribution terms, including the base OS/drivers and any bundled third-party assets. The present review excludes proprietary game inputs and documents the source chain; it is not a completed legal audit or a blanket assurance that every historical binary/asset may be republished. If a component's rights or source coverage are unclear, obtain the required permission/source or omit it from public distribution.

The owner has made the repository public. A public source repository and a firmware release are separate distributions. Sharing source with the required licenses is supported; do not present the project as an official upstream release.

The newer complete images (G350 0.1.14, RG35XX SP 0.2.3, RG SP 0.3.1) bundle PortMaster/runtime components. Preserve the in-package licenses and pinned source/build records for those components too. The H700 adaptation uses nextui-portmaster-h700 v0.4.0 and NextUI h700-rc11 SDL libraries; pins and the Slot adapter are under ports/h700. No supplied games or commercial disc scans are added.

The selected NextUI h700-rc11 runtime libraries come with the upstream PolyForm Noncommercial 1.0.0 license and MinUI-derived notices. They are not covered by this repository's GPL grant. Their license/notice copies are retained under ports/h700/licenses and in the H700 runtime packages. Do not present the combined H700 firmware as unrestricted GPL-only software or assume commercial redistribution is permitted.
