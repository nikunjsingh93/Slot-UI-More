# Slot 1.1.15 — complete images

All three supported devices use one release number and the title `Slot <model> 1.1.15 — Complete Image`. Tags are `g350-v1.1.15`, `rgsp-v1.1.15` and `rg35xxsp-v1.1.15`. GitHub supports one repository-wide Latest badge; G350 receives it and the README links all three current releases.

This is a release packaging change. Full disk image bytes, emulator cores, settings and gameplay behavior match the previously verified G350 0.1.14, RG SP 0.3.1 and RG35XX SP 0.2.3 builds. Their internal frontend version strings remain unchanged. New compressed archives contain `.img` files with consistent 1.1.15 names. Fresh-image hardware validation remains pending; H700 native Ports remain experimental.

Download the model-specific `.img.zip`, extract, and flash the `.img` to a backed-up card. No earlier base or follow-up installation is needed. Optional `system-files.zip` archives are for existing matching cards and source rebuild inputs; they are not flashable images and must never be written with an image writer. Follow each device guide when using them.

Superseded release pages are removed after verifying all replacement downloads. Git tags and commit history remain as provenance. Corresponding historical core source bundles are carried forward under `slot-1.1.15-core-sources.zip` and `slot-1.1.15-additional-core-sources.zip` on the G350 release. Historical records retain original version numbers and hashes. No game data, BIOS, private executable, saves or commercial disc scans are included.
