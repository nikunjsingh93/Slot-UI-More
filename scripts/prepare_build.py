from pathlib import Path
import hashlib, json, subprocess, sys, zipfile

repo=Path(sys.argv[1]);build=Path(sys.argv[2])
downloads=json.loads((repo/'docs/downloads.json').read_text())
for name in ['slot-g350-0.1.8-parallel-n64-update.zip','slot-g350-0.1.9.2-sources.zip']:
    p=repo/'backups'/name
    h=hashlib.sha256()
    with p.open('rb') as f:
        while chunk:=f.read(8*1024*1024):h.update(chunk)
    assert h.hexdigest()==downloads[name]['sha256'],f'Archive checksum mismatch: {name}'
with zipfile.ZipFile(repo/'backups/slot-g350-0.1.8-parallel-n64-update.zip') as z:
    with z.open('SYSTEM') as inp,(build/'BASE-SYSTEM').open('wb') as out:
        import shutil
        shutil.copyfileobj(inp,out,8*1024*1024)
if not (build/'root').exists():
    subprocess.run(['unsquashfs','-processors','4','-d',str(build/'root'),str(build/'BASE-SYSTEM')],check=True)
with zipfile.ZipFile(repo/'backups/slot-g350-0.1.9.2-sources.zip') as z:
    for name in ['flycast-v2.5-source.tar.gz','ppsspp-v1.20.4-source.tar.gz']:
        target=build/name
        with z.open('g350/'+name) as inp,target.open('wb') as out:shutil.copyfileobj(inp,out,8*1024*1024)
        subprocess.run(['tar','-xzf',str(target),'-C',str(build/'core-source'),'--exclude=*/build','--exclude=*/build-*'],check=True)
print('PASS verified backups; preserved stable root and complete pinned core sources extracted')
