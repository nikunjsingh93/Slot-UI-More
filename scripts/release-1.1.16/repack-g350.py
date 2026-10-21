from pathlib import Path
import zipfile,hashlib,json,subprocess,shutil
w=Path('/tmp/slot-release-116');root=w/'g350-root';out=Path('/outputs/release-1.1.16');repo=Path('/work/github-g350')
def sha(p,kind='sha256'):
 h=hashlib.new(kind)
 with p.open('rb') as f:
  for b in iter(lambda:f.read(4<<20),b''):h.update(b)
 return h.hexdigest()
original=json.loads(Path('/outputs/slot-g350-1.1.15-manifest.json').read_text())
with zipfile.ZipFile('/outputs/slot-g350-1.1.15-system-files.zip') as z:
 for name in ['SYSTEM','KERNEL','KERNEL.md5']:
  with z.open(name) as src,(w/name).open('wb') as dst:shutil.copyfileobj(src,dst,4<<20)
assert sha(w/'SYSTEM')==original['system_sha256']
subprocess.run(['unsquashfs','-processors','4','-d',str(root),str(w/'SYSTEM')],check=True)
assert sha(root/'usr/bin/slot-g350')==original['frontend_sha256']
new=w/'g350-target/device/slot';shutil.copyfile(new,root/'usr/bin/slot-g350');(root/'usr/bin/slot-g350').chmod(0o755)
(root/'usr/share/slot/VERSION').write_text('1.1.16\n')
for name,digest in original['core_sha256'].items():assert sha(root/'usr/share/slot'/name)==digest,name
subprocess.run(['python3',str(repo/'scripts/verify_device_entrypoint.py'),str(root/'usr/bin/slot-g350')],check=True)
subprocess.run(['python3',str(repo/'firmware/ports/test_slot_directories.py'),str(root)],check=True)
system=w/'SYSTEM-1.1.16'
subprocess.run(['mksquashfs',str(root),str(system),'-noappend','-comp','gzip','-b','131072','-processors','4','-no-progress'],check=True)
archive=w/'g350-system-files.zip'
with zipfile.ZipFile(archive,'w',zipfile.ZIP_STORED) as z:
 z.write(system,'SYSTEM');z.writestr('SYSTEM.md5',sha(system,'md5')+'  target/SYSTEM\n')
 for name in ['KERNEL','KERNEL.md5']:z.write(w/name,name)
m=json.loads((w/'g350-system-manifest.json').read_text())
m.update(version='1.1.16',firmware_version='1.1.16',release_version='1.1.16',frontend_sha256=sha(new),system_size=system.stat().st_size,system_sha256=sha(system),system_md5=sha(system,'md5'),source_archive='slot-1.1.16-sources.zip',release_note='Accepted G350 N64/Dreamcast cores and boot files retained; frontend version synchronized and incompatible N64 state sizes rejected safely.',runtime_hardware_tested=False,physical_validation='Predecessor runtime owner-tested. Version bump and N64 state-size guard checked in software; fresh boot pending.')
(w/'g350-system-manifest.json').write_text(json.dumps(m,indent=2)+'\n')
p=w/'image-inputs.json';inputs=json.loads(p.read_text());inputs['g350']['update']={'path':str(archive),'sha256':sha(archive)};p.write_text(json.dumps(inputs,indent=2)+'\n')
print('PASS G350 repack: frontend 1.1.16, exact accepted cores/kernel, clean directory rules.',flush=True)
