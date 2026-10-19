# Artwork sources, permissions and privacy

The web app is a Slot-derived GPL-3.0 application, with JSZip under its bundled MIT/GPLv3 license choice. Preserve the supplied licenses and source when distributing the code. The code license does not license the third-party game artwork or imply an endorsement by its owners.

## Where artwork comes from

| Provider | Used for | How it is fetched |
|---|---|---|
| [LaunchBox Games Database](https://gamesdb.launchbox-app.com/) | GB/GBC/GBA/SNES/N64 cartridge scans, Dreamcast/PSP disc scans, supplemental logos and front covers | Image identifiers from the embedded metadata catalogue; images are fetched from images.launchbox-app.com through [wsrv.nl](https://wsrv.nl/) for browser-compatible access and resizing |
| [PSX1G1B1C1D](https://github.com/UnluckyForSome/PSX1G1B1C1D) | PS1 disc scans | On-demand images from that repository through raw.githubusercontent.com |
| [Libretro Thumbnails](https://github.com/libretro-thumbnails/libretro-thumbnails) | Fallback game logos, front covers and title screens | On-demand images from its platform repositories through raw.githubusercontent.com |

Each ready label links to its image source and identifies when fallback artwork was used. The published app contains code, fonts, cartridge outlines and catalogue metadata; it does not bundle downloaded third-party cover/disc artwork or ROMs. The project's README screenshot gallery uses original fictional demo labels instead.

## What this permits

No blanket permission covering every third-party image has been established. Public availability of an image, a source credit, or a software repository license is not evidence that the game publisher's artwork is freely reusable. Provider terms and the underlying copyright holder's rights still apply. Do not assume downloaded labels can be sold, redistributed as an artwork pack, bundled with firmware or publicly republished without permission or another applicable legal basis.

Personal or noncommercial use is not automatically a copyright exception. Applicable exceptions vary by country and circumstances. The [U.S. Copyright Office's fair-use guide](https://www.copyright.gov/fair-use/) explains that fair use involves multiple factors and a case-specific analysis. This project does not promise that every image use qualifies. For redistribution or commercial use, obtain permission or use artwork you own or that has an appropriate explicit license.

## Local files and network requests

Folder scanning and label generation run in your browser. The studio does not upload ROM contents, saves, playlists or complete directory listings to this project. Filenames are matched locally against the embedded catalogue. Small PS1/DC playlists are read locally to group discs; ROM contents are not read for matching.

Fetching artwork sends requests to the selected provider and, for LaunchBox images, wsrv.nl. Those services receive your IP address and the requested image URL; a GitHub image URL can include the matched game title. This is not a wholly offline or anonymous workflow. GitHub Pages serves the app itself. No project backend, account system, analytics or game-file upload endpoint is added.

## GitHub Pages

The hosted studio is [nikunjsingh93.github.io/Slot-UI-More](https://nikunjsingh93.github.io/Slot-UI-More/). The workflow builds only the standalone HTML and license files, without firmware images, personal files or downloaded game artwork. Changes under studio/ on main automatically redeploy it. Chrome/Edge can request permission to write labels to a chosen folder; other browsers can use folder selection and ZIP export where supported.
