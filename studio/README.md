# Slot Automatic Artwork Studio v4

Open `Slot-Auto-Artwork-Studio-v4.html` in Chrome or Edge. Choose **Slot folder** once, let artwork download, then click **Save ready labels to Slot**. Alternatively download the labels ZIP and merge its `Labels` folder into Slot. Restart Slot to see the new labels. No firmware update or reflash is needed.

| System folder under Slot | Supported game files | Label PNG |
|---|---|---|
| Games/SNES | sfc, smc | 224×139 |
| Games/PS1 | chd, cue, m3u, pbp, exe | 312×312 |
| Games/N64 | z64, n64, v64 | 180×190 |
| Games/DC | chd, gdi, cdi, cue, m3u | 312×312 |
| Games/PSP | iso, cso, pbp | 312×312 disc print, shown inside Slot's 312×224 UMD shell |

Files are written to `Labels/<system>/<exact game filename stem>.png`. Existing labels are kept by default. Enable **Replace existing labels** if you want to replace older logos, then save the previews. GBA/GB/GBC remain handled by the original Cart Studio.

The default **Original disc / cartridge label** prefers real disc printing and cartridge scans. PS1 scans use [PSX1G1B1C1D](https://github.com/UnluckyForSome/PSX1G1B1C1D); other physical scans use [LaunchBox](https://gamesdb.launchbox-app.com/). Logos and covers from [Libretro thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails) and LaunchBox provide fallback artwork. The preview identifies fallbacks and links its source. LaunchBox images pass through [wsrv.nl](https://wsrv.nl/) for browser-compatible image access.

The 2026-10-09 catalog includes physical media for 740 Dreamcast and 1,023 PSP titles, in addition to SNES, PS1 and N64. Not every title or region has a scan. For example, Virtua Tennis 3's available PSP scan is Australian. The studio uses it when a US scan is absent and identifies the region. Unusual filenames may need manual title selection. **Adjust scan crop** lets you correct scan alignment; PSP defaults crop around the printed disc to avoid duplicating the photographed UMD case.

ROM contents are never read or uploaded for matching. Small PS1/DC playlists are read locally to group referenced discs. Choose the Slot folder itself or the card folder containing Slot. The HTML embeds its catalog and ZIP library; no local server or account is needed. Internet access is required to fetch artwork. Offline failures can be retried.

Browser validation covered live scans for all five systems, your Dreamcast/PSP tennis filenames, Tekken 6 artwork, original disc transparency and UMD previews, PS1/DC playlists, unchanged existing labels, exact output filenames/dimensions, direct writes, ZIP export, folder-upload fallback, logos when scans fail, offline retry, and mobile layout. This update changes the artwork studio only.

The ZIP contains the standalone HTML, this guide, and editable source with a portable Python build script and JSZip's license. To rebuild, run `python source/build.py --output Slot-Auto-Artwork-Studio-v4.html` from the extracted folder. Python uses only its standard library.
