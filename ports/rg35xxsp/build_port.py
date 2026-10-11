"""Rebuild the SP frontend/image from the pinned shared source and known cores.

Run inside an AArch64 Debian Bullseye/Rust 1.96 container. Read README.md.
"""
import argparse, hashlib, io, json, shutil, subprocess, tarfile, zipfile
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--repo', type=Path, required=True)
parser.add_argument('--work', type=Path, required=True)
parser.add_argument('--baseos', type=Path, required=True)
parser.add_argument('--g350-update', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
repo, w, out = args.repo.resolve(), args.work.resolve(), args.output.resolve()
port = Path(__file__).resolve().parent
assert subprocess.check_output(['uname','-m'], text=True).strip() == 'aarch64'
assert not w.exists(), 'Use a new empty work directory; existing data is never removed.'
w.mkdir(parents=True); out.mkdir(parents=True, exist_ok=True)

def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for block in iter(lambda:f.read(4<<20), b''): h.update(block)
    return h.hexdigest()

def run(*cmd, **kw):
    return subprocess.run(cmd, check=True, **kw)

assert sha(args.baseos) == '263e2fd6e7df1d177f3de4d79acb75c7b222c4ad17c78ec2cf858e87f0c89738'
assert sha(args.g350_update) == '9581d66652c0950cba02090abbcc1af41c48a231738ff8901183a6f9473e4e8a'
shutil.copy2(args.baseos, w/'baseos-rg35xxsp-1.3.0.img.zip')
with zipfile.ZipFile(args.baseos) as z:
    names=[i for i in z.infolist() if i.filename.endswith('.img')]
    assert len(names)==1
    with z.open(names[0]) as src, (w/'baseos-rg35xxsp.img').open('wb') as dst:
        shutil.copyfileobj(src, dst, 4<<20)
with (w/'baseos-rg35xxsp.img').open('rb') as src, (w/'rootfs-original.ext4').open('wb') as dst:
    src.seek(434176*512); remaining=536870912
    while remaining:
        block=src.read(min(4<<20,remaining)); assert block
        dst.write(block); remaining-=len(block)
shutil.copy2(port/'partitions.json', w/'partitions.json')
archive=subprocess.check_output(['git','-C',str(repo),'archive','e787972:slot'])
(w/'slot').mkdir()
with tarfile.open(fileobj=io.BytesIO(archive)) as t:
    for info in t:
        # This is our pinned Git tree, containing only source and card helpers.
        dest=(w/'slot'/info.name).resolve()
        assert dest.is_relative_to(w/'slot')
        t.extract(info, w/'slot')
run('patch','-p1','-i',str(port/'frontend.patch'),cwd=w/'slot')
for filename in ['package_sp.py','slot-sp-install','launch_frontend.sh','RG35XXSP-START-HERE.txt']:
    shutil.copy2(port/filename, w/filename)
with zipfile.ZipFile(args.g350_update) as z:
    names=[i for i in z.infolist() if Path(i.filename).name=='SYSTEM']
    assert len(names)==1
    with z.open(names[0]) as src, (w/'G350-reference.SYSTEM').open('wb') as dst:
        shutil.copyfileobj(src,dst,4<<20)
assert sha(w/'G350-reference.SYSTEM') == 'cb08010b9554c602489c404557e572f508782b54899bf8769bc87bd348c20309'
# Only emulators/assets/notices are reused; no G350 kernel or drivers enter SP.
run('unsquashfs','-d',str(w/'g350-reference'),str(w/'G350-reference.SYSTEM'),'usr/share/slot')
run('cargo','build','--locked','--profile','device','--bin','slot','--no-default-features','--features','device',cwd=w/'slot')
shutil.copy2(w/'slot/target/device/slot',w/'slot-device')
shutil.copy2(repo/'studio/Slot-Auto-Artwork-Studio-v4.html',out/'Slot-Auto-Artwork-Studio-v4.html')
run('python3',str(w/'package_sp.py'),'--work',str(w),'--repo',str(repo),'--cores',str(w/'g350-reference/usr/share/slot'),'--output',str(out))
print('PASS SP image rebuilt. Binary hashes may differ with toolchain/build date; hardware test still required.')
