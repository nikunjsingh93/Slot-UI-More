from pathlib import Path
import hashlib,json,shutil
W=Path(__file__).resolve().parent
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
payload=W/'update'; (payload/'System').mkdir(parents=True,exist_ok=True)
shutil.copyfile(W/'slot-device',payload/'System/slot')
shutil.copyfile(W/'launch_frontend.sh',payload/'System/launch_frontend.sh')
old=json.loads((W/'card-backup/System/slot-sp-build.json').read_text())
for name,digest in old['cores'].items():assert sha(W/'card-backup/System'/name)==digest
manifest=dict(old)
manifest.update(version='0.3.0',device='Anbernic RG SP',base_firmware='RG SP BaseOS 1.3.0',
    update_baseline='RG35XX SP Slot 0.2.2',frontend_sha256=sha(W/'slot-device'),
    baseos_archive_sha256=sha(W/'baseos-rgsp-1.3.0.img.zip'),source_archive='slot-rgsp-0.3.0-sources.zip',
    panel='720x480 (3:2)',console_aspect='4:3 centered; PSP 480:272 letterboxed',
    gb_height_fit='160:144, 533/534 physical columns at 480-pixel height')
(payload/'System/slot-sp-build.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
guide='''Slot RG SP 0.3.0 - converted from the existing RG35XX SP card

Use this card in the RG SP TF1/OS slot. Keep TF2 empty for the first test.
GB, GBC, GBA, SNES, PS1, N64, Dreamcast and PSP are retained.
Games, Labels, BIOS, Saves, States, Config, Tools and Wallpapers stay on TF1.
Hold Menu for one second to leave a game and return to Slot.
PS1/SNES/N64/DC/PSP: Menu+X saves state; Menu+Y loads state.
N64/DC/PSP: Menu+Select toggles D-pad / analog control. N64 starts analog.
GB/GBC: L1 alternates screen stretch and full-height aspect fit; R1 resets.
Tap Menu on the shelf to choose Original, Sharp Pixels, Smooth, LCD or CRT.

RG SP native panel: 720x480. PS1/SNES/N64/DC retain centered 4:3 geometry;
PSP retains its 480:272 geometry. Discs use the native canvas without the
RG35XX SP's horizontal compensation, so they remain round.

The original expanded card partition table is preserved. Conversion replaces
only RG SP boot0/bootloader/resources/kernel and active rootfs, followed by
the frontend, launcher and build manifest. Persistent OS userdata is retained.
All existing emulator core binaries/assets remain unchanged.
No first-boot game partition format or frontend reinstall is requested.
Boot and game performance on the physical RG SP still need user validation.

This card is now model-specific RG SP firmware. Do not use it in RG35XX SP.
Keep the local verified card backup and boot-region backup for rollback.
'''
(payload/'RGSP-START-HERE.txt').write_text(guide,encoding='utf-8')
files=[dict(path=str(p.relative_to(payload)).replace('\\','/'),sha256=sha(p),size=p.stat().st_size) for p in sorted(payload.rglob('*')) if p.is_file()]
(W/'payload-files.json').write_text(json.dumps(files,indent=2),encoding='utf-8')
print('PASS staged frontend/launcher/manifest; all nine existing cores verified')
