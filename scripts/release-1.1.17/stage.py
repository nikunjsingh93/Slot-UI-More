"""Composition record for 1.1.17; explicit clean 1.1.16 caches, never a game card.

Run in the ARM64 builder after build_extra_cores.py and frontend builds.
Cache paths describe this release's verified inputs; adjust when rebuilding.
"""
from pathlib import Path
import hashlib,json,shutil,subprocess,zipfile

repo=Path('/work/github-g350');old=Path('/tmp/slot-release-116');work=Path('/tmp/slot-release-117')
work.mkdir(exist_ok=True);output=Path('/outputs/release-1.1.17');output.mkdir(exist_ok=True)
cores=Path('/tmp/slot-extra/cores')
systems=['GBA','GB','GBC','PS1','SNES','N64','DC','PSP','PORTS','NES','MD','SMS','GG','PCE','ARCADE']
def sha(p,kind='sha256'):
    h=hashlib.new(kind)
    with p.open('rb') as f:
        for b in iter(lambda:f.read(4<<20),b''):h.update(b)
    return h.hexdigest()
pins=json.loads((cores/'manifest.json').read_text())
inputs=json.loads((old/'image-inputs.json').read_text())
for model in ['rgsp','rg35xxsp']:
    dest=work/(model+'-overlay');assert not dest.exists()
    shutil.copytree(old/(model+'-overlay'),dest)
    system=dest/'System';m=json.loads((system/'slot-sp-build.json').read_text())
    for name,digest in m['cores'].items():assert sha(system/name)==digest,name
    frontend=Path('/tmp/slot-sp-036/target/device/slot')
    shutil.copyfile(frontend,system/'slot')
    for pin in pins:
        name=pin['name']+'_libretro.so';assert sha(cores/name)==pin['binary_sha256']
        shutil.copyfile(cores/name,system/name);m['cores'][name]=pin['binary_sha256']
    shutil.copytree(cores/'notices',system/'additional-core-notices',ignore=shutil.ignore_patterns('*.tar.gz'))
    shutil.copyfile(cores/'FBNeo-Arcade.dat',system/'FBNeo-Arcade.dat')
    m.update(version='1.1.17',base_firmware='1.1.17',systems=systems,
             frontend_sha256=sha(frontend),source_archive='slot-1.1.17-sources.zip',
             additional_core_sources='slot-1.1.17-additional-core-sources.zip',hardware_tested=False,
             physical_validation='New system support and fresh image first boot not hardware-tested; predecessor controls/N64/Dreamcast runtime preserved.')
    (system/'slot-sp-build.json').write_text(json.dumps(m,indent=2)+'\n')
    inputs[model].update(version='1.1.17',frontend_payload=str(dest))

root=work/'g350-root';assert not root.exists()
shutil.copytree(Path('/tmp/slot-extra/test-package/g350-root'),root,symlinks=True)
share=root/'usr/share/slot';m=json.loads((old/'g350-system-manifest.json').read_text())
for name,digest in m['core_sha256'].items():assert sha(share/name)==digest,name
frontend=old/'g350-target/device/slot';shutil.copyfile(frontend,root/'usr/bin/slot-g350')
shutil.copyfile(repo/'firmware/slot-start',root/'usr/bin/slot-start')
(root/'usr/bin/slot-start').chmod(0o755);(root/'usr/bin/slot-g350').chmod(0o755)
(share/'VERSION').write_text('1.1.17\n')
tools=share/'Tools';tools.mkdir(exist_ok=True)
shutil.copyfile(repo/'studio/Slot-Auto-Artwork-Studio-v5.html',tools/'Slot-Auto-Artwork-Studio-v5.html')
shutil.copytree(cores/'notices',share/'additional-core-notices',dirs_exist_ok=True,ignore=shutil.ignore_patterns('*.tar.gz'))
for pin in pins:
    name=pin['name']+'_libretro.so';assert sha(share/name)==pin['binary_sha256']
    m['core_sha256'][name]=pin['binary_sha256']
subprocess.run(['python3',str(repo/'scripts/verify_device_entrypoint.py'),str(root/'usr/bin/slot-g350')],check=True)
subprocess.run(['python3',str(repo/'firmware/ports/test_slot_directories.py'),str(root)],check=True)
system=work/'SYSTEM'
subprocess.run(['mksquashfs',str(root),str(system),'-noappend','-comp','gzip','-b','131072','-processors','4','-no-progress'],check=True)
readback=work/'g350-readback'
files=['usr/bin/slot-g350','usr/bin/slot-start','usr/share/slot/VERSION','usr/share/slot/Tools/Slot-Auto-Artwork-Studio-v5.html']+['usr/share/slot/'+name for name in m['core_sha256']]
subprocess.run(['unsquashfs','-processors','4','-no-progress','-d',str(readback),str(system),*files],check=True)
for file in files:assert sha(root/file)==sha(readback/file),file
archive=work/'g350-system-files.zip'
with zipfile.ZipFile(archive,'w',zipfile.ZIP_STORED) as z:
    z.write(system,'SYSTEM');z.writestr('SYSTEM.md5',sha(system,'md5')+'  target/SYSTEM\n')
    for name in ['KERNEL','KERNEL.md5']:z.write(old/name,name)
m.update(version='1.1.17',firmware_version='1.1.17',release_version='1.1.17',systems=systems,
         frontend_sha256=sha(frontend),system_size=system.stat().st_size,system_sha256=sha(system),system_md5=sha(system,'md5'),
         source_archive='slot-1.1.17-sources.zip',additional_core_sources='slot-1.1.17-additional-core-sources.zip',
         hardware_tested=False,runtime_hardware_tested=False,emulator_cores_unchanged=False,
         release_note='Adds NES, MD, SMS, GG, PCE and FBNeo; preserves all nine previously accepted cores; expanded Artwork Studio installed on first boot.',
         physical_validation='New system support and fresh image first boot pending handheld tests.')
(work/'g350-system-manifest.json').write_text(json.dumps(m,indent=2)+'\n')
inputs['g350'].update(version='1.1.17',update={'path':str(archive),'sha256':sha(archive)},system_manifest=str(work/'g350-system-manifest.json'))
(work/'image-inputs.json').write_text(json.dumps(inputs,indent=2)+'\n')
print('PASS staged synchronized frontends, 13 cores, clean runtime and model-specific image inputs',flush=True)
