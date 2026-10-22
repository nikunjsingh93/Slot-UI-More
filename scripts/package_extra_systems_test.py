"""Package 1.1.17 test system files; never opens a physical SD card.

G350 needs a clean expanded 1.1.16 root plus its manifest, not a personal card.
H700 packages are overlays for existing Slot 1.1.16 installs, not flash images.
"""
import argparse,hashlib,json,shutil,subprocess,zipfile
from pathlib import Path

def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as f:
        for b in iter(lambda:f.read(4<<20),b''):h.update(b)
    return h.hexdigest()

def zip_files(path,files):
    with zipfile.ZipFile(path,'w',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
        for source,name in files:z.write(source,name)
    with zipfile.ZipFile(path) as z:assert z.testzip() is None

def main():
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['g350-root','g350-manifest','g350-frontend','h700-frontend','cores','output','stage']:
        p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--resume',action='store_true',help='Reuse this script\'s partially completed staging root')
    p.add_argument('--model',choices=['all','h700'],default='all',help='Use h700 to refresh only the two SP overlays after an H700-only change')
    a=p.parse_args();repo=Path(__file__).resolve().parents[1]
    a.output.mkdir(parents=True,exist_ok=True)
    if a.stage.exists() and not a.resume:raise RuntimeError('Use a fresh staging directory or --resume')
    a.stage.mkdir(parents=True,exist_ok=True)
    version='1.1.17';codes=['NES','MD','SMS','GG','PCE','ARCADE']
    records=json.loads((a.cores/'manifest.json').read_text())
    for r in records:assert sha(a.cores/(r['name']+'_libretro.so'))==r['binary_sha256']
    old=json.loads(a.g350_manifest.read_text())
    for name,digest in old['core_sha256'].items():assert sha(a.g350_root/'usr/share/slot'/name)==digest
    root=a.stage/'g350-root'
    if not root.exists():shutil.copytree(a.g350_root,root,symlinks=True)
    else:
        assert a.resume and (root/'usr/share/slot/VERSION').read_text().strip()==version
    shutil.copyfile(a.g350_frontend,root/'usr/bin/slot-g350');(root/'usr/bin/slot-g350').chmod(0o755)
    shutil.copyfile(repo/'firmware/slot-start',root/'usr/bin/slot-start');(root/'usr/bin/slot-start').chmod(0o755)
    share=root/'usr/share/slot';(share/'VERSION').write_text(version+'\n')
    for r in records:shutil.copyfile(a.cores/(r['name']+'_libretro.so'),share/(r['name']+'_libretro.so'))
    shutil.copytree(a.cores/'notices',share/'additional-core-notices',dirs_exist_ok=True,ignore=shutil.ignore_patterns('*.tar.gz'))
    shutil.copyfile(a.cores/'FBNeo-Arcade.dat',share/'FBNeo-Arcade.dat')
    for name,digest in old['core_sha256'].items():assert sha(share/name)==digest,name
    system=a.stage/'SYSTEM'
    if not (a.resume and system.exists()):
        subprocess.run(['mksquashfs',str(root),str(system),'-noappend','-comp','gzip','-b','131072','-processors','4','-no-progress'],check=True)
    subprocess.run(['unsquashfs','-s',str(system)],check=True)
    # Read back both the frontend and all new/accepted cores from the actual image.
    expected={'usr/bin/slot-g350':sha(a.g350_frontend),
              'usr/bin/slot-start':sha(repo/'firmware/slot-start'),
              'usr/share/slot/VERSION':sha(share/'VERSION'),
              'usr/share/slot/FBNeo-Arcade.dat':sha(a.cores/'FBNeo-Arcade.dat')}
    expected.update({'usr/share/slot/'+r['name']+'_libretro.so':r['binary_sha256'] for r in records})
    expected.update({'usr/share/slot/'+name:digest for name,digest in old['core_sha256'].items()})
    readback=a.stage/'readback'
    subprocess.run(['unsquashfs','-processors','4','-no-progress','-force','-dest',str(readback),str(system),*expected],check=True)
    for internal,digest in expected.items():assert sha(readback/internal)==digest,internal
    md5=a.stage/'SYSTEM.md5';md5.write_text(hashlib.md5(system.read_bytes()).hexdigest()+'  target/SYSTEM\n')
    studio=repo/'studio/Slot-Auto-Artwork-Studio-v5.html';guide=repo/'docs/ADDITIONAL-SYSTEMS.md'
    common=[(studio,'Tools/'+studio.name),(guide,'ADDITIONAL-SYSTEMS.md')]
    # Notices also accompany G350's outer package, separately from its SYSTEM.
    common += [(f,'additional-core-notices/'+str(f.relative_to(a.cores/'notices'))) for f in (a.cores/'notices').rglob('*') if f.is_file() and not f.name.endswith('.tar.gz')]
    previous=json.loads((a.output/'test-build-manifest.json').read_text()) if a.model=='h700' else {}
    manifest={'version':version,'status':'test build','hardware_tested':False,'systems_added':codes,
              'accepted_g350_cores_unchanged':old['core_sha256'],'additional_cores':records,
              'g350_frontend_sha256':sha(a.g350_frontend),'h700_frontend_sha256':sha(a.h700_frontend),
              'system_sha256':sha(system),'studio_sha256':sha(studio),'packages':previous.get('packages',{})}
    for model in ['g350','rgsp','rg35xxsp']:
        if model=='g350' and a.model=='h700':
            assert previous['g350_frontend_sha256']==sha(a.g350_frontend)
            assert previous['system_sha256']==sha(system) and previous['studio_sha256']==sha(studio)
            continue
        readme=a.stage/(model+'-README.txt')
        if model=='g350':
            readme.write_text('SLOT G350 1.1.17 TEST SYSTEM FILES - NOT A FLASH IMAGE\n\nFor an existing Slot G350 card. Power off and back up its SYSTEM/SYSTEM.md5.\nCopy SYSTEM and SYSTEM.md5 onto the boot partition, replacing that pair.\nCopy the Slot folder from this ZIP into GAMES, merging Tools only.\nKeep your existing kernel, games, labels, BIOS, saves and settings.\nSafely eject and boot. New cores/folders are provisioned automatically.\nSee ADDITIONAL-SYSTEMS.md for games, arcade sets, controls and validation limits.\nNo physical boot/gameplay verification of this build has been performed.\n')
            files=[(system,'SYSTEM'),(md5,'SYSTEM.md5')]+[(f,'Slot/'+n) for f,n in common]
        else:
            readme.write_text('SLOT '+model.upper()+' 1.1.17 TEST SYSTEM FILES - NOT A FLASH IMAGE\n\nFor an existing Slot 1.1.16 '+model+' card. Power off. Back up System/slot.\nExtract this ZIP to BASEOS, merging System/Tools folders. Do not format the card.\nKeep existing files when there is no replacement in this ZIP. Games, labels,\nBIOS, saves, native launchers and settings are not included or overwritten.\nSafely eject and boot. Empty system folders are created by the frontend.\nSee ADDITIONAL-SYSTEMS.md for games, arcade sets, controls and validation limits.\nNo physical boot/gameplay verification of this build has been performed.\n')
            files=[(a.h700_frontend,'System/slot')]+common
            files += [(a.cores/(r['name']+'_libretro.so'),'System/'+r['name']+'_libretro.so') for r in records]
            files += [(a.cores/'FBNeo-Arcade.dat','System/FBNeo-Arcade.dat')]
            files += [(f,'System/additional-core-notices/'+str(f.relative_to(a.cores/'notices'))) for f in (a.cores/'notices').rglob('*') if f.is_file() and not f.name.endswith('.tar.gz')]
        archive=a.output/f'slot-{model}-{version}-test-system-files.zip'
        files += [(readme,'START-HERE.txt')];zip_files(archive,files)
        manifest['packages'][model]={'file':archive.name,'sha256':sha(archive),'bytes':archive.stat().st_size}
    source_files=[]
    for pin in records:
        folder=a.cores/'notices'/pin['name'];archive=folder/pin['archive']
        assert sha(archive)==pin['sha256'];source_files.append((archive,archive.name))
    source_files += [(repo/'docs/additional-core-sources.json','sources.json'),(repo/'scripts/build_extra_cores.py','build_extra_cores.py')]
    zip_files(a.output/f'slot-{version}-additional-core-sources.zip',source_files)
    shutil.copyfile(studio,a.output/studio.name)
    (a.output/'test-build-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    sums=''.join(sha(f)+'  '+f.name+'\n' for f in sorted(a.output.iterdir()) if f.is_file() and f.name!='SHA256SUMS.txt')
    (a.output/'SHA256SUMS.txt').write_text(sums)
    print('PASS three model test packages, G350 SYSTEM readbacks, retained nine accepted cores and archive CRCs')

if __name__=='__main__':main()
