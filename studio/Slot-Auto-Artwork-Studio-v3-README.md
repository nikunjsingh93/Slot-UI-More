# Automatic artwork studio v3

Open the standalone HTML in Chrome or Edge. Choose the **Slot folder** containing Games and Labels. SNES, PS1 and N64 filenames are matched locally and artwork is fetched automatically. Click **Save ready labels to Slot**, or download the labels ZIP and merge its Labels folder into Slot. Restart the handheld.

**Original disc / cartridge label** is the default. To upgrade your existing logo labels, enable **Replace existing labels**. Labels are preserved until you explicitly save replacements. The studio supplies previews, source links, match correction and optional crop controls. ROM contents are never uploaded or read for matching; only filenames, small local PS1 playlists and existing label PNGs are used.

Original disc printing is preferred from [PSX1G1B1C1D](https://github.com/UnluckyForSome/PSX1G1B1C1D). SNES and N64 cartridge scans use the public [LaunchBox Games Database](https://gamesdb.launchbox-app.com/) metadata, delivered through the public [wsrv.nl](https://wsrv.nl/) image processor because the source image host does not permit browser CORS reads. Only the public artwork URL is sent to that processor. Images are fetched lazily after you select a folder. No account or API key is needed.

If a usable physical scan is unavailable, the studio falls back to [Libretro thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails), preferring a game logo, then a cover/title image. Fallbacks are identified in the preview. Offline downloads fail visibly and can be retried. Catalog snapshot: 2026-10-08; it includes 827 SNES titles with cartridge-front images, 424 N64 titles with cartridge-front images, and 1,735 PS1 disc entries. Those counts do not guarantee matching or availability for every ROM filename or region. Images belong to their respective owners; provider terms still apply.

## Formats and dimensions

| System | Game files | Output PNG |
|---|---|---|
| SNES | .sfc, .smc | 224×139 |
| PS1 | .chd, .cue, .m3u, .pbp, .exe | 312×312, transparent spindle hole for scans |
| N64 | .z64, .n64, .v64 | 180×190 |

Output: `Labels/<system>/<exact game filename stem>.png`. Scan Games directly under each system folder. PS1 playlists suppress their referenced disc entries and use Disc 1 art. A separately listed Disc 2/3 game keeps its disc number. Region is inferred from the filename; unspecified regions prefer North America. Unusual names require a title selection. Cartridge scan layouts differ, so automatic crops can need adjustment; expand **Adjust scan crop** to view the complete scan and change X/Y/width/height percentages.

Use Slot G350 0.1.3 for N64 and complete printed-disc previews. GBA/GB/GBC remain handled by the original Cart Studio.

## Source and testing

The ZIP includes the HTML, separate application source, bundled catalog, build script, browser tests and JSZip's license. The HTML embeds its own catalog and JSZip, so no local server is required. Its source uses standard browser file/canvas APIs. Chrome/Edge allow direct folder writes after choosing a folder; other browsers can use the folder picker and ZIP export.

Browser tests cover live physical-media images from a local-file origin, all three systems, playlist grouping, untouched ROM contents, existing-label preservation, PNG dimensions and names, direct-save and ZIP export, crop adjustment, logo fallback, offline retry, write failures and mobile layout.
