from pathlib import Path
import hashlib,json,zipfile
W=Path(__file__).resolve().parent;OUT=W.parent.parent/'outputs'
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
name='slot-rgsp-0.3.0'
guide='''RG SP conversion package 0.3.0

This package converts an existing expanded RG35XX SP Slot 0.2.2 card to RG SP.
It is NOT a flashable .img and is NOT an ordinary System-folder-only update.
The boot0, bootloader/device tree, boot resources, kernel and active rootfs
must also be replaced. Do not merely copy the frontend to an unconverted card.

The delivered card has been converted directly with a guarded administrator
installer. Its existing GPT and data partition are preserved. Your games,
labels, BIOS, saves, states, wallpapers and settings stay at the card root.
All nine existing emulator binaries and PPSSPP assets remain byte-identical.
Use the converted card in RG SP TF1, not RG35XX SP. Keep TF2 empty initially.
Read RGSP-START-HERE.txt for controls. Physical boot validation is pending.

The installer in the source archive is intentionally pinned to the card that
was inspected in this session (disk 1, USB SD/MMC reader, 62,226,694,144 bytes).
It must not be run on another disk without reinspection and a verified backup.
Boot offsets/region sizes/hashes are in boot-regions.json. No GPT/data offsets
are written. A verified complete local card backup and raw original system
region backup are retained under work/rgsp-migration on this laptop.

Rebuild: extract the complete frontend source; build in AArch64 Debian Bullseye
with Rust using cargo build --locked --profile device --bin slot
--no-default-features --features device. The original SP source tree is retained.
Official model-specific BaseOS RG SP 1.3.0 archive, verified SHA256:
39b184867c6628a39d0b197d70b06c07eb74fed2f3d54a1b27e372bac562df12
https://github.com/pvaibhav/BaseOS/releases/tag/v1.3.0
prepare_boot.py adds only libgomp1 for Flycast and TF1-first frontend selection.
It extracts the official RG SP boot chain starting at byte 8192 and patched p5.
prepare_payload.py retains all core revisions from the backed-up SP manifest.
Slot/source licenses are in the frontend tree; BaseOS build source is included.
Existing core source/rebuild notices remain on the card under System/licenses.
No games, labels, proprietary BIOS, saves or card backup are included here.
'''
(OUT/(name+'-README.txt')).write_text(guide,encoding='utf-8')
manifest=json.loads((W/'update/System/slot-sp-build.json').read_text())
manifest['boot_regions']=json.loads((W/'boot-regions.json').read_text())
(OUT/(name+'-manifest.json')).write_text(json.dumps(manifest,indent=2),encoding='utf-8')
archive=OUT/(name+'-conversion.zip')
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
    for file in ['rgsp-boot-region.bin','rootfs-rgsp.ext4','boot-regions.json']:
        z.write(W/file,file)
    for p in sorted((W/'update').rglob('*')):
        if p.is_file():z.write(p,'card-files/'+p.relative_to(W/'update').as_posix())
    z.write(OUT/(name+'-README.txt'),'README.txt')
source=OUT/(name+'-sources.zip')
with zipfile.ZipFile(source,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as z:
    for p in sorted((W/'slot').rglob('*')):
        if p.is_file() and not {'target','.git'}.intersection(p.relative_to(W/'slot').parts):z.write(p,'slot/'+p.relative_to(W/'slot').as_posix())
    for p in sorted(W.iterdir()):
        if p.is_file() and p.suffix in ['.py','.ps1','.log'] and p.name not in ['card-backup-copy.log','installation.log']:
            z.write(p,'conversion/'+p.name)
    for f in ['boot-regions.json','payload-files.json','launch_frontend.sh']:
        z.write(W/f,'conversion/'+f)
    z.write(W.parent/'rg35xxsp-v0100/baseos-build-source.tar.gz','baseos-build-source.tar.gz')
    z.write(OUT/(name+'-README.txt'),'README.txt')
for p in [archive,source]:
    with zipfile.ZipFile(p) as z:assert z.testzip() is None
checks=[archive,source,OUT/(name+'-README.txt'),OUT/(name+'-manifest.json')]
(OUT/(name+'-SHA256SUMS.txt')).write_text(''.join(f'{sha(p)}  {p.name}\n' for p in checks),encoding='utf-8')
print(json.dumps({p.name:dict(bytes=p.stat().st_size,sha256=sha(p)) for p in checks},indent=2),flush=True)
