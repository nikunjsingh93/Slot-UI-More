from pathlib import Path
import hashlib, json, subprocess, sys, zipfile

root=Path(sys.argv[1]).resolve();out=Path(sys.argv[2]).resolve();out.mkdir(parents=True,exist_ok=True)
repo=Path(__file__).resolve().parent.parent
base=json.loads((repo/'docs/G350-0.1.8-manifest.json').read_text())
def digest(p,kind='sha256'):
    h=hashlib.new(kind)
    with p.open('rb') as f:
        while chunk:=f.read(8*1024*1024):h.update(chunk)
    return h.hexdigest()
for name,expected in base['core_sha256'].items():
    assert digest(root/'usr/share/slot'/name)==expected, f'Stable core changed: {name}'
for executable in ['usr/bin/slot-g350','usr/share/slot/flycast_libretro.so','usr/share/slot/ppsspp_libretro.so']:
    result=subprocess.run(['chroot',str(root),'/usr/bin/ldd','/'+executable],capture_output=True,text=True)
    assert result.returncode==0 and 'not found' not in result.stdout+result.stderr,result.stdout+result.stderr
subprocess.run(['sh','-n',str(root/'usr/bin/slot-start')],check=True)
system=out/'SYSTEM'
subprocess.run(['mksquashfs',str(root),str(system),'-noappend','-comp','gzip','-b','131072','-processors','4','-no-progress'],check=True)
md5=digest(system,'md5');pair=out/'SYSTEM.md5';pair.write_text(md5+'  target/SYSTEM\n')
archive=out/'slot-g350-0.1.9.3-rebuilt-update.zip'
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as z:z.write(system,'SYSTEM');z.write(pair,'SYSTEM.md5')
with zipfile.ZipFile(archive) as z:assert z.testzip() is None
manifest=dict(version='0.1.9.3-rebuilt',system_sha256=digest(system),system_md5=md5,archive_sha256=digest(archive),core_sha256={p.name:digest(p) for p in (root/'usr/share/slot').glob('*.so')})
(out/'rebuilt-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('PASS rebuilt matching SYSTEM pair and ZIP; dependencies and preserved stable cores verified')
