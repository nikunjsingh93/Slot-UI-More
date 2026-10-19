"""Stage the pinned H700 manager runtime, without games or GPU drivers.

Usage: prepare_runtime.py PORTS.pak.zip NextUI-v6.14.0-h700-rc11.zip OUTPUT
Run on Linux. Obtain the two archives from the releases in runtime-pins.json.
"""
from pathlib import Path
import hashlib, io, json, shutil, subprocess, sys, tarfile, zipfile

pak, nextui, dest = map(Path, sys.argv[1:])
here=Path(__file__).resolve().parent
pins=json.loads((here/'runtime-pins.json').read_text())
for path,key in [(pak,'ports_pak_sha256'),(nextui,'nextui_zip_sha256')]:
    h=hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda:f.read(4<<20),b''):h.update(block)
    assert h.hexdigest()==pins[key],path
assert not dest.exists(),'Use a new output directory'
dest.mkdir(parents=True)
with zipfile.ZipFile(pak) as z:
    for entry in z.infolist():
        p=Path(entry.filename)
        assert p.parts[0]=='PORTS.pak' and '..' not in p.parts
        if 'port-fixes' in p.parts:continue
        target=dest.joinpath(*p.parts[1:])
        if entry.is_dir():target.mkdir(parents=True,exist_ok=True)
        else:
            target.parent.mkdir(parents=True,exist_ok=True)
            target.write_bytes(z.read(entry))
with zipfile.ZipFile(nextui) as outer:
    with zipfile.ZipFile(io.BytesIO(outer.read('MinUI.zip'))) as z:
        lib=dest/'system-lib';lib.mkdir()
        for entry in z.infolist():
            if entry.filename.startswith('.system/h700/lib/') and not entry.is_dir():
                (lib/Path(entry.filename).name).write_bytes(z.read(entry))
for category in ['bin','lib']:
    archive=dest/'files'/(category+'.tar.gz')
    with tarfile.open(archive) as t:
        members=t.getmembers()
        assert all(not Path(m.name).is_absolute() and '..' not in Path(m.name).parts and (m.isfile() or m.isdir()) for m in members)
        t.extractall(dest/category)
    archive.unlink()
launch=dest/'launch.sh';launch.write_bytes(launch.read_bytes().replace(b'\r\n',b'\n'))
subprocess.run(['patch','-p1','-i',str(here/'runtime-adapter.patch')],cwd=dest,check=True)
shutil.copytree(here/'licenses',dest/'licenses',dirs_exist_ok=True)
for p in dest.rglob('*'):
    if p.is_dir():p.chmod(0o755)
    elif p.is_file():
        with p.open('rb') as f:prefix=f.read(4)
        p.chmod(0o755 if prefix.startswith((b'#!',b'\x7fELF')) else 0o644)
shutil.copyfile(here/'runtime-pins.json',dest/'slot-runtime-provenance.json')
print('PASS pinned manager, private libraries, H700 SDL and Slot launch adapter staged')
