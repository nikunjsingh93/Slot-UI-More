# Slot Automatic Artwork Studio v5

**[Open the hosted studio](https://nikunjsingh93.github.io/Slot-UI-More/)** in Chrome or Edge. It runs locally in your browser after loading. [Image sources, permissions and privacy](ARTWORK-SOURCES.md) explain the external artwork requests and copyright limits.

Built on [Slot by Brandon Kowalski](https://github.com/BrandonKowalski/slot), with its original cartridge artwork credited to Markus upstream. This expanded studio is independently maintained. See [project credits](../CREDITS.md) and the retained GPL/component licenses.

Open `Slot-Auto-Artwork-Studio-v5.html` in Chrome or Edge. Choose **Slot folder** once, let artwork download, then click **Save ready labels to Slot**. Alternatively download the labels ZIP and merge its `Labels` folder into Slot. Restart Slot to see the new labels. No firmware update or reflash is needed. GB, GBC and GBA now work in this same app; no separate Cart Studio is needed for their artwork.

On G350, choose `GAMES/Slot`. On RG SP / RG35XX SP, choose the `BASEOS` folder containing `Games` and `Labels`. The studio also accepts a parent folder containing `Slot`. Preview results before saving and correct uncertain names using the search on each game tile.

| System folder under Slot | Supported game files | Label PNG |
|---|---|---|
| Games/GB | gb | 220×195 |
| Games/GBC | gbc, gb | 220×195 |
| Games/GBA | gba | 266×138 |
| Games/SNES | sfc, smc | 224×139 |
| Games/PS1 | chd, cue, m3u, pbp, exe | 312×312 |
| Games/N64 | z64, n64, v64 | 180×190 |
| Games/DC | chd, gdi, cdi, cue, m3u | 312×312 |
| Games/PSP | iso, cso, pbp | 312×312 disc print, shown inside Slot's 312×328 UMD shell |

Files are written to `Labels/<system>/<exact game filename stem>.png`. Existing labels are kept by default, including ones previously made by Cart Studio. Enable **Replace existing labels** if you want to replace older logos, then save the previews. Scanning does not alter your games, saves or shell settings.

The default **Original disc / cartridge label** prefers real disc printing and cartridge scans. PS1 scans use [PSX1G1B1C1D](https://github.com/UnluckyForSome/PSX1G1B1C1D); other physical scans use [LaunchBox](https://gamesdb.launchbox-app.com/). Logos and covers from [Libretro thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails) and LaunchBox provide fallback artwork. The preview identifies fallbacks and links its source. LaunchBox images pass through [wsrv.nl](https://wsrv.nl/) for browser-compatible image access.

The original 2026-10-09 catalog includes physical media for 740 Dreamcast and 1,023 PSP titles, in addition to SNES, PS1 and N64. The 2026-10-19 extension adds 3,864 GB, 2,961 GBC and 8,288 GBA matching titles, including 704, 337 and 1,386 titles with cartridge scans respectively. Game Boy title lists use pinned Libretro repository revisions; physical scan identifiers come from the existing 2026-10-09 LaunchBox metadata snapshot. Not every title or region has a scan. The studio prefers your filename's region, then another available region, and identifies the source. Unusual filenames may need manual title selection. **Adjust scan crop** corrects scan alignment; Game Boy scans crop to the printed label rather than saving a whole cartridge photograph.

ROM contents are never read or uploaded for matching. Small PS1/DC playlists are read locally to group referenced discs. Choose the Slot folder itself or the card folder containing Slot. The HTML embeds its catalog and ZIP library; no local server or account is needed. Internet access is required to fetch artwork. Offline failures can be retried.

Browser validation for v5 covered all eight systems, exact output names/dimensions, preservation of existing labels, direct writes, ZIP export, parent-folder scanning, physical-scan fallbacks and mobile layout. Live requests fetched physical cartridge artwork for Super Mario Land (GB), Pokémon Crystal Version (GBC) and Mario Kart: Super Circuit (GBA). ROM reads are rejected by the test fixtures. This changes the artwork studio only; console firmware is unchanged.

The ZIP contains the standalone HTML, this guide, and editable source with a portable Python build script and JSZip's license. To rebuild, run `python source/build.py --output Slot-Auto-Artwork-Studio-v5.html` from the extracted folder. Python uses only its standard library. `source/add_gameboy_catalog.py` documents regeneration of the Game Boy metadata from GitHub tree exports and LaunchBox Metadata.zip. `source/cart-shells.json` contains the existing Slot SVG cartridge outlines for the browser preview, under the same GPL license as the frontend.

UMD correction (0.1.9.4): the round window and flared lower shell match the physical UMD front. New PSP scan labels preserve the printed hub rather than cutting a CD spindle hole through it. Existing PNGs still work; enable replacement in Studio to regenerate an older scan if its printed centre is incomplete.
