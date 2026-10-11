"""Package Slot on the verified model-specific BaseOS image; no ROMs required.

Run in the ARM64 build container with /work and /outputs mounts.
"""
from pathlib import Path
import argparse, hashlib, json, shutil, subprocess, tarfile, zipfile

parser = argparse.ArgumentParser()
parser.add_argument('--work', type=Path, default=Path('/work/rg35xxsp-v0100'))
parser.add_argument('--output', type=Path, default=Path('/outputs'))
parser.add_argument('--cores', type=Path, default=Path('/tmp/g350-v019-root/usr/share/slot'))
parser.add_argument('--repo', type=Path, default=Path('/work/github-g350'))
args = parser.parse_args()
W = args.work.resolve()
OUT = args.output.resolve()
OUT.mkdir(parents=True, exist_ok=True)
BASE = W / 'baseos-rg35xxsp.img'
ROOT = W / 'rootfs-slot-sp.ext4'
NAME = 'slot-rg35xxsp-0.2.0'
CORES = args.cores.resolve()
REPO = args.repo.resolve()

def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout

def sha(p):
    with p.open('rb') as f:
        return digest(f)

def digest(f):
    h = hashlib.sha256()
    for block in iter(lambda: f.read(4<<20), b''):
        h.update(block)
    return h.hexdigest()

def write(p, text):
    p.parent.mkdir(parents=True, exist_ok=True)
    with p.open('w', encoding='utf-8', newline='\n') as f:
        f.write(text)

def copied(src, dst):
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)

def dump(internal, dest):
    dest.unlink(missing_ok=True)
    run('debugfs', '-R', f'dump {internal} {dest}', str(ROOT))
    assert dest.is_file(), f'missing rootfs file: {internal}'

assert sha(W/'baseos-rg35xxsp-1.3.0.img.zip') == '263e2fd6e7df1d177f3de4d79acb75c7b222c4ad17c78ec2cf858e87f0c89738'
stage = W / 'payload'
system = stage / 'System'
system.mkdir(parents=True, exist_ok=True)
copied(W/'slot-device', system/'slot')
(system/'slot').chmod(0o755)
for f in (W/'slot/card/System').glob('*.sh'):
    write(system/f.name, f.read_text(encoding='utf-8'))
    (system/f.name).chmod(0o755)
copied(W/'launch_frontend.sh', system/'launch_frontend.sh')
for f in CORES.glob('*_libretro.so'):
    copied(f, system/f.name)
shutil.copytree(CORES/'PPSSPP', system/'PPSSPP', dirs_exist_ok=True)
assets = sorted(str(f.relative_to(system/'PPSSPP')) for f in (system/'PPSSPP').rglob('*') if f.is_file())
assert len(assets) == 189
write(system/'ppsspp-assets.txt', '\n'.join(assets)+'\n')
licenses = system/'licenses'
shutil.copytree(CORES/'notices', licenses, dirs_exist_ok=True)
shutil.copytree(W/'slot/licenses', licenses/'slot', dirs_exist_ok=True)
copied(W/'slot/LICENSE', licenses/'Slot-GPL-3.0.txt')
copied(Path('/usr/share/doc/libgomp1/copyright'), licenses/'libgomp1-copyright.txt')
for f in REPO.joinpath('patches').glob('*'):
    if f.is_file() and f.suffix in ['.txt', '.json', '.meta']:
        copied(f, licenses/'core-revisions'/f.name)
write(licenses/'SOURCES.txt', '''Slot RG35XX SP 0.2.0 source and license information
Slot: GPL-3.0-or-later, based on BrandonKowalski/slot v1.4.0 plus this
repository's G350 e787972 baseline and the RG35XX SP overlay.
RG35XX SP frontend, patches and packaging: companion
slot-rg35xxsp-0.2.0-sources.zip and ports/rg35xxsp in
https://github.com/nikunjsingh93/slot-ui-g350
BaseOS: official RG35XX SP v1.3.0, commit
7dec5cba47db9b77b18b546a52acc51b1df64088:
https://github.com/pvaibhav/BaseOS/releases/tag/v1.3.0
Its kernel, modules, Mali driver and stock runtime are retained.
The companion source archive includes BaseOS build source; original stock
firmware/prepared parts remain available through BaseOS's fetch scripts.
All nine emulator binaries are byte-identical to G350 0.1.9.4.
Their complete corresponding source, patches, submodules and restore
instructions accompany the v0.1.9.2 source asset (cores were unchanged):
https://github.com/nikunjsingh93/slot-ui-g350/releases/tag/v0.1.9.2
Asset: slot-g350-0.1.9.2-sources.zip. Sign in to access this private repo.
See included notices and core-revisions for licenses/revisions.
libgomp1 10.2.1-6 (AArch64) is added for Flycast. GCC runtime exception
and GPL notices are in libgomp1-copyright.txt; Debian corresponding source:
https://snapshot.debian.org/package/gcc-10/10.2.1-6/
No games, proprietary BIOS, private media, saves or credentials included.
''')
manifest = {
    'version': '0.2.0', 'device': 'Anbernic RG35XX SP', 'experimental': True,
    'hardware_tested': False, 'baseos': '1.3.0',
    'baseos_archive_sha256': sha(W/'baseos-rg35xxsp-1.3.0.img.zip'),
    'baseos_commit': '7dec5cba47db9b77b18b546a52acc51b1df64088',
    'g350_source_baseline': 'e787972', 'g350_firmware_baseline': '0.1.9.4',
    'architecture': 'aarch64', 'graphics': 'H700 vendor fbdev EGL/GLES',
    'systems': ['GB', 'GBC', 'GBA', 'SNES', 'PS1', 'N64', 'DC', 'PSP'],
    'frontend_sha256': sha(system/'slot'),
    'cores': {f.name: sha(f) for f in sorted(system.glob('*_libretro.so'))},
    'ppsspp_assets': len(assets), 'games_included': False,
    'source_archive': NAME+'-sources.zip',
    'analog_toggle': 'Menu + Select on N64/DC/PSP; N64 starts analog',
    'quit': 'Menu held 1000 ms',
    'minimum_card_gb': 16, 'storage': 'TF1 data root; TF2 empty',
}
write(system/'slot-sp-build.json', json.dumps(manifest, indent=2)+'\n')
copied(W/'RG35XXSP-START-HERE.txt', stage/'RG35XXSP-START-HERE.txt')
copied(OUT/'Slot-Auto-Artwork-Studio-v4.html', stage/'Tools/Slot-Auto-Artwork-Studio-v4.html')
files = sorted(f for f in stage.rglob('*') if f.is_file() and f.name not in ['payload.sha256', 'system.sha256'])
write(stage/'system.sha256', ''.join(f'{sha(f)}  {f.relative_to(stage)}\n' for f in files if f.is_relative_to(system)))
write(stage/'payload.sha256', ''.join(f'{sha(f)}  {f.relative_to(stage)}\n' for f in files+[stage/'system.sha256']))
payload = W/'payload.tar.gz'
with tarfile.open(payload, 'w:gz', compresslevel=6) as t:
    for f in sorted(stage.iterdir()):
        t.add(f, arcname=f.name)
print('Verified payload prepared:', payload.stat().st_size, flush=True)

shutil.copy2(W/'rootfs-original.ext4', ROOT)
rcs = W/'rcS-patched'
dump('/etc/init.d/rcS', rcs)
text = rcs.read_text().replace('/dev/mmcblk1p1 /dev/mmcblk0p7', '/dev/mmcblk0p7 /dev/mmcblk1p1')
anchor = '# Older rootfs images may lack the alias baked into current builds.'
assert text.count(anchor) == 1
text = text.replace(anchor, '''# Install the bundled Slot frontend only after TF1 expansion has completed.
if [ "$USB_STORAGE_MODE" -eq 0 ] && grep -q '^/dev/mmcblk0p7 /mnt/sdcard ' /proc/mounts; then
    if ! /usr/sbin/slot-sp-install > /mnt/sdcard/slot-install.log 2>&1; then
        /usr/bin/baseos-splash --important -1 "SLOT INSTALL FAILED: CHECK SLOT-INSTALL.LOG" 2>/dev/null
        while :; do sleep 60; done
    fi
fi
'''+anchor)
assert text.index('/usr/sbin/expand-storage') < text.index('/usr/sbin/slot-sp-install')
write(rcs, text)
front = W/'frontend-session-patched'
dump('/usr/sbin/frontend-session', front)
write(front, front.read_text().replace('/dev/mmcblk1p1 /dev/mmcblk0p7', '/dev/mmcblk0p7 /dev/mmcblk1p1'))
add = [
    ('/usr/share/slot-sp/payload.tar.gz', payload, False),
    ('/usr/sbin/slot-sp-install', W/'slot-sp-install', True),
    ('/etc/init.d/rcS', rcs, True),
    ('/usr/sbin/frontend-session', front, True),
    ('/lib/aarch64-linux-gnu/libgomp.so.1', Path('/usr/lib/aarch64-linux-gnu/libgomp.so.1').resolve(), False),
]
commands = ['mkdir /usr/share/slot-sp']
for internal, source, executable in add:
    if internal in ['/etc/init.d/rcS', '/usr/sbin/frontend-session']:
        commands.append(f'rm {internal}')
    commands.append(f'write {source} {internal}')
    commands.append(f'set_inode_field {internal} mode 0100{755 if executable else 644}')
write(W/'rootfs-commands', '\n'.join(commands)+'\n')
run('debugfs', '-w', '-f', str(W/'rootfs-commands'), str(ROOT))
for internal, source, _ in add:
    dest = W/'root-verify'
    dump(internal, dest)
    assert sha(dest) == sha(source), f'rootfs readback failed: {internal}'
write(W/'rootfs-check.log', run('e2fsck', '-fn', str(ROOT)))
print('PASS rootfs filesystem and all file readbacks', flush=True)
parts = json.loads((W/'partitions.json').read_text())
p = next(p for p in parts if p['number'] == 5)
assert ROOT.stat().st_size == p['bytes'] == 536870912
image = OUT/(NAME+'.img')
shutil.copy2(BASE, image)
with image.open('r+b') as dst, ROOT.open('rb') as src:
    dst.seek(p['first']*512)
    shutil.copyfileobj(src, dst, 4<<20)
write(W/'gpt-check.log', run('sgdisk', '-v', str(image)))
with image.open('rb') as final, BASE.open('rb') as base:
    start = p['first']*512
    end = start+p['bytes']
    for a, b in [(0, start), (end, image.stat().st_size)]:
        final.seek(a); base.seek(a); left = b-a
        while left:
            n = min(left, 4<<20)
            assert final.read(n) == base.read(n)
            left -= n
manifest['vendor_boot_regions_byte_identical'] = True
manifest['image_sha256'] = sha(image)
manifest['image_bytes'] = image.stat().st_size
zpath = OUT/(NAME+'.img.zip')
with zipfile.ZipFile(zpath, 'w', zipfile.ZIP_DEFLATED, compresslevel=6) as z:
    z.write(image, image.name)
with zipfile.ZipFile(zpath) as z:
    assert z.testzip() is None
    with z.open(image.name) as f:
        assert digest(f) == manifest['image_sha256']
manifest['zip_sha256'] = sha(zpath)
manifest['zip_bytes'] = zpath.stat().st_size
write(OUT/(NAME+'-manifest.json'), json.dumps(manifest, indent=2)+'\n')
copied(W/'RG35XXSP-START-HERE.txt', OUT/(NAME+'-START-HERE.txt'))
write(OUT/(NAME+'-SHA256SUMS.txt'), f"{manifest['image_sha256']}  {image.name}\n{manifest['zip_sha256']}  {zpath.name}\n")
print('PASS GPT, unchanged non-rootfs regions, ZIP CRC and decompressed image SHA256', flush=True)
print(json.dumps(manifest, indent=2))
