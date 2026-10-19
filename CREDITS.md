# Credits and attribution

## Original Slot

This project is derived from **[Slot by Brandon Kowalski](https://github.com/BrandonKowalski/slot)**, the original Game Boy-centric firmware/frontend for the RG SP. Its original design, frontend code and upstream contributors are the foundation of this adaptation. The upstream README credits **Markus** for its skeuomorphic artwork; that credit is preserved here as well. See [the original project and its credits](https://github.com/BrandonKowalski/slot#ai-disclosure).

The retained baseline is Slot commit `d249d7731e52fabf3272119d4d3fd4702107951a`; later device and integration changes are recorded in this repository's history and source/provenance records. This adaptation adds device support, additional systems, native Ports integration and a combined automatic Artwork Studio. It is independently maintained, not an official upstream release or an endorsement by Brandon Kowalski or the other credited projects. Attribution/hosting documentation was updated on 2026-10-19.

Preserve the original GPL-3.0 license text, existing copyright/license notices and modification records when sharing derived code. When distributing covered binaries, also supply the corresponding source and build/install material required by the applicable license. A credit line alone does not satisfy those requirements. [LICENSE](LICENSE), [the retained upstream license](slot/LICENSE), [source revisions](docs/SOURCE-REVISIONS.txt) and [sharing notes](docs/SHARING.md) provide the existing records. Component-specific licenses remain applicable.

## Other projects and artwork providers

- **Emulators and device firmware:** the mGBA, gpSP, Snes9x, PCSX ReARMed, ParaLLEl/Mupen64Plus, Flycast and PPSSPP authors, together with the underlying OS/device projects. Their original licenses, notices, patches and pinned source records remain part of the source and runtime packages.
- **PortMaster:** [PortsMaster](https://github.com/PortsMaster/PortMaster-GUI), individual port authors, and the H700 integration/runtime projects identified under [ports/h700](ports/h700/README.md).
- **Artwork Studio ZIP support:** [JSZip](https://stuk.github.io/jszip/), with its [bundled license](studio/source/JSZip-LICENSE.markdown).
- **On-demand artwork:** LaunchBox Games Database, PSX1G1B1C1D contributors, Libretro Thumbnails contributors, and the original publishers/artists whose images are referenced. **wsrv.nl** provides image caching/resizing for the LaunchBox fetch path. Attribution does not grant artwork redistribution rights; see [sources and permissions](studio/ARTWORK-SOURCES.md).
- **Fonts and other assets:** retain the component-specific license files alongside the assets in slot/crates/slot-ui/assets and the remaining upstream license directories.
