"""Run against the real BaseOS BusyBox userspace; no block devices used."""
from pathlib import Path
import hashlib, os, shutil, subprocess

W = Path('/work/rg35xxsp-v0100')
root = Path('/tmp/rg35xxsp-root')
payload = W/'payload.tar.gz'
shutil.copy2(payload, root/'payload.tar.gz')
shutil.copy2(W/'slot-sp-install', root/'installer')
def run(folder, broken=False):
    env = dict(os.environ, SLOT_INSTALL_ROOT='/'+folder,
               SLOT_INSTALL_PAYLOAD='/broken.tar.gz' if broken else '/payload.tar.gz')
    return subprocess.run(['chroot', str(root), '/bin/sh', '/installer'], env=env, capture_output=True, text=True)

clean = root/'installer-clean'
assert (clean/'System/.slot-rg35xxsp-0.2.0').is_file()
for category in ['Games','Labels','Saves','States']:
    for platform in ['GB','GBC','GBA','SNES','PS1','N64','DC','PSP']:
        assert (clean/category/platform).is_dir()
for f in (clean/'System/PPSSPP').rglob('*'):
    if f.is_file():
        assert (clean/'BIOS/PPSSPP'/f.relative_to(clean/'System/PPSSPP')).read_bytes() == f.read_bytes()
assert run('installer-clean').returncode == 0
print('PASS clean first boot, all eight folders, 189 PPSSPP assets, idempotence')

retry = root/'installer-retry'
retry.mkdir(exist_ok=True)
for file, data in [('System/slot', b'interrupted frontend'), ('.slot-sp-installing', b''),
                   ('Games/N64/User.z64', b'private local test sentinel'),
                   ('Saves/DC/User.sav', b'keep save'), ('Config/user.txt', b'keep config'),
                   ('Labels/PSP/User.png', b'keep label'), ('BIOS/PPSSPP/lang/en_US.ini', b'keep asset')]:
    dst = retry/file; dst.parent.mkdir(parents=True, exist_ok=True); dst.write_bytes(data)
before = {str(p.relative_to(retry)): p.read_bytes() for p in retry.rglob('*') if p.is_file() and 'System' not in p.parts and p.name != '.slot-sp-installing'}
# Gzip checksum damage must fail without committing the completion marker.
data = bytearray(payload.read_bytes()); data[-8] ^= 0xff
(root/'broken.tar.gz').write_bytes(data)
assert run('installer-retry', broken=True).returncode != 0
assert not (retry/'System/.slot-rg35xxsp-0.2.0').exists()
assert run('installer-retry').returncode == 0
for rel, data in before.items():
    assert (retry/rel).read_bytes() == data, rel
assert (retry/'System/slot').read_bytes() == (clean/'System/slot').read_bytes()
assert not (retry/'.slot-sp-installing').exists()
print('PASS corrupt-payload failure, interrupted install recovery, preservation of games/saves/labels/config/BIOS')

foreign = root/'installer-foreign/System'
foreign.mkdir(parents=True, exist_ok=True)
(foreign/'slot').write_bytes(b'other frontend')
assert run('installer-foreign').returncode == 0
assert (foreign/'slot').read_bytes() == b'other frontend'
assert not (foreign.parent/'.slot-sp-installing').exists()
print('PASS unrelated frontend left untouched')
