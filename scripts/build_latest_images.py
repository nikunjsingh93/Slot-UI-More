"""Package validated frontends into model-specific fresh-install images.

Run in the ARM64 Bullseye build container. Inputs are local verified artifacts;
this script never opens a physical disk or includes user games/card backups.
"""
from pathlib import Path
import argparse, hashlib, json, os, shutil, subprocess, tarfile, zipfile
import struct

parser=argparse.ArgumentParser()
parser.add_argument('--work',type=Path,default=Path('/work/latest-releases'))
parser.add_argument('--outputs',type=Path,default=Path('/outputs'))
parser.add_argument('--inputs',type=Path,required=True,help='JSON file defining verified artifact paths')
parser.add_argument('--model',choices=['g350','rg35xxsp','rgsp','all'],default='all')
args=parser.parse_args();W=args.work;OUT=args.outputs
W.mkdir(parents=True,exist_ok=True);OUT.mkdir(parents=True,exist_ok=True)
inputs=json.loads(args.inputs.read_text())
def write(p,s):
    p.parent.mkdir(parents=True,exist_ok=True)
    with p.open('w',encoding='utf-8',newline='\n') as f:f.write(s)
def sha(p,kind='sha256'):
    h=hashlib.new(kind)
    with p.open('rb') as f:
        for b in iter(lambda:f.read(4<<20),b''):h.update(b)
    return h.hexdigest()
def run(*cmd):
    r=subprocess.run(cmd,check=True,capture_output=True,text=True)
    return r.stdout+r.stderr
def checked(item):
    p=Path(item['path']);assert sha(p)==item['sha256'],p
    return p
def dump(root,internal,p):
    p.unlink(missing_ok=True)
    run('debugfs','-R',f'dump {internal} {p}',str(root));assert p.exists(),internal
def ranges_equal(a,b,ranges):
    with a.open('rb') as x,b.open('rb') as y:
        for start,end in ranges:
            x.seek(start);y.seek(start);left=end-start
            while left:
                n=min(4<<20,left);assert x.read(n)==y.read(n),(start,end);left-=n
def zero_free_fat_clusters(image,offset):
    # Deleted old SYSTEM clusters otherwise remain in the fresh image and
    # almost double its ZIP. Zero only FAT-marked free data clusters; never
    # modify allocated files, FAT tables, reserved sectors or boot records.
    with image.open('r+b') as f:
        f.seek(offset);b=f.read(512)
        sector=struct.unpack_from('<H',b,11)[0];spc=b[13]
        reserved=struct.unpack_from('<H',b,14)[0];fats=b[16]
        root_entries=struct.unpack_from('<H',b,17)[0]
        total=struct.unpack_from('<H',b,19)[0] or struct.unpack_from('<I',b,32)[0]
        fat16=struct.unpack_from('<H',b,22)[0]
        fat_size=fat16 or struct.unpack_from('<I',b,36)[0]
        root_sectors=(root_entries*32+sector-1)//sector
        data_sector=reserved+fats*fat_size+root_sectors
        clusters=(total-data_sector)//spc
        assert sector==512 and spc and clusters>=4085,'Expected FAT16/FAT32 image'
        f.seek(offset+reserved*sector);table=f.read(fat_size*sector)
        step=2 if fat16 else 4;fmt='<H' if fat16 else '<I'
        free=[]
        for cluster in range(2,clusters+2):
            value=struct.unpack_from(fmt,table,cluster*step)[0]
            if not fat16:value&=0x0fffffff
            if value==0:free.append(cluster)
        groups=[]
        for c in free:
            if groups and c==groups[-1][1]+1:groups[-1][1]=c
            else:groups.append([c,c])
        zeros=bytes(4<<20)
        for first,last in groups:
            f.seek(offset+(data_sector+(first-2)*spc)*sector)
            left=(last-first+1)*spc*sector
            while left:
                n=min(len(zeros),left);f.write(zeros[:n]);left-=n
        f.flush();os.fsync(f.fileno())
    return len(free)*spc*sector
def publish_local(model,version,image,manifest,guide,validation):
    name=f'slot-{model}-{version}'
    archive=OUT/(name+'.img.zip')
    with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=3) as z:z.write(image,image.name)
    with zipfile.ZipFile(archive) as z:
        assert z.testzip() is None
        with z.open(image.name) as f:
            h=hashlib.sha256()
            for b in iter(lambda:f.read(4<<20),b''):h.update(b)
            assert h.hexdigest()==sha(image)
    manifest.update(image_sha256=sha(image),image_bytes=image.stat().st_size,
                    zip_sha256=sha(archive),zip_bytes=archive.stat().st_size,
                    fresh_install=True,games_included=False,proprietary_bios_included=False,
                    hardware_tested=False,physical_validation='Fresh image first boot pending; existing frontend builds tested separately')
    write(OUT/(name+'-fresh-manifest.json'),json.dumps(manifest,indent=2)+'\n')
    write(OUT/(name+'-START-HERE.txt'),guide)
    write(OUT/(name+'-VALIDATION.txt'),validation)
    print(f'PASS {model} {version}: fresh image, filesystems, installed payload and ZIP SHA256',flush=True)
def h700(model):
    info=inputs[model];version=info['version'];name=f'slot-{model}-{version}'
    d=W/model;d.mkdir(parents=True,exist_ok=True)
    archive=checked(info['base']);base=d/'official.img'
    with zipfile.ZipFile(archive) as z:
        names=[n for n in z.namelist() if n.endswith('.img')];assert len(names)==1
        with z.open(names[0]) as src,base.open('wb') as dst:shutil.copyfileobj(src,dst,4<<20)
    # Official RG SP and RG35XX SP 1.3.0 use this same audited geometry.
    start=222298112;size=536870912;end=start+size
    root=d/'rootfs.ext4'
    with base.open('rb') as src,root.open('wb') as dst:
        src.seek(start);left=size
        while left:
            b=src.read(min(4<<20,left));assert b;dst.write(b);left-=len(b)
    assert f'BASEOS_TARGET={model}\n' in run('debugfs','-R','cat /etc/baseos-release',str(root))
    stage=d/'payload';assert not stage.exists(),'Use a fresh work directory for packaging'
    shutil.copytree(Path(info['payload_base']),stage)
    # Staging comes only from the original clean SP firmware payload.
    for forbidden in ['Games','Labels','Saves','States','Config','BIOS']:
        assert not (stage/forbidden).exists(),forbidden
    # Current overlays include the native-port supervisor and pinned runtime.
    shutil.copytree(Path(info['frontend_payload'])/'System',stage/'System',dirs_exist_ok=True)
    for p in (stage/'System').glob('*.sh'):p.chmod(0o755)
    (stage/'System/slot').chmod(0o755)
    manifest=json.loads((stage/'System/slot-sp-build.json').read_text())
    assert manifest['version']==version
    assert sha(stage/'System/slot')==manifest['frontend_sha256']
    for f,h in manifest['cores'].items():assert sha(stage/'System'/f)==h,f
    for asset in (stage/'System/ppsspp-assets.txt').read_text().splitlines():
        assert (stage/'System/PPSSPP'/asset).is_file(),f'Missing bundled free PSP asset: {asset}'
    label=info['display_name'];guide_name=info['guide_name']
    guide=f'''Slot {label} {version} - fresh installation

Download slot-{model}-{version}.img.zip and extract its .img. Flash that .img
to a 16 GB or larger SD card with your usual image writer. Flashing erases the
target card: back up any games, labels and saves first. Use this image only
on {label}. Insert into TF1/OS; keep TF2 empty for the initial boot.

Let the first boot finish expanding the data partition and installing Slot.
Do not copy games before this completes: first boot formats the empty data
partition. A restart or a longer first boot is expected. Keep power available.
After reaching Slot, shut down and connect the card to your computer.

At the BASEOS root, add games to Games/GB, Games/GBC, Games/GBA, Games/SNES,
Games/PS1, Games/N64, Games/DC and Games/PSP. Artwork goes under Labels in
the corresponding system folders, with matching game filename stems.
Copy your existing BIOS, Saves, States and Config folders back if restoring.
Keep this image's System folder. G350 SYSTEM updates are incompatible.
No games or proprietary BIOS are included; PPSSPP's free assets are bundled.
The artwork tool is under Tools/Slot-Auto-Artwork-Studio-v5.html.

Controls: hold Menu alone one second to return to Slot. Console save/load
states: Menu+X / Menu+Y. GB/GBC/GBA: Select+R1 / Select+L1 (prompt chord).
N64/DC/PSP: Menu+Select toggles D-pad digital/analog; N64 starts analog.
GB/GBC: L1 alternates full-screen stretch and full-height aspect fit;
R1 restores default. Tap Menu at the shelf to select Shader: Original,
Sharp Pixels, Smooth, LCD or CRT. The repository README lists all hotkeys.

This fresh image contains the current frontend and eight emulated systems and Ports directly;
no older base image or follow-up System update is required.
PortMaster is bundled in System/PortMasterH700. Open the Ports shelf to install
games; internet is needed for downloads and missing runtimes. For Wi-Fi, copy
System/wifi-example.conf to Config/wifi.conf, fill in your network name/password
and reboot. Check Config/wifi.log if needed. Game data stays
in ports/.ports; top-level launch scripts appear under ports. Labels for these
entries go in Labels/PORTS with matching names. Hold Menu one second to quit.
The H700 native runtime is new and still requires physical testing. Runtime, image,
installer and rendering checks pass. Physical first boot of this newly packaged
image still needs validation; no new hardware performance claim is made.
'''
    for p in stage.glob('*START-HERE.txt'):p.unlink()
    write(stage/guide_name,guide)
    write(stage/'System/licenses/LATEST-BUILD.txt',f'{label} {version}: corresponding updated frontend and packaging scripts are in the matching source ZIP and repository. BaseOS is official {model} 1.3.0. Existing emulator cores/assets are unchanged; see SOURCES.txt and the v0.1.9.2 corresponding core source release.\n')
    # Fresh payload manifest describes the image, not the earlier card conversion.
    manifest.update(base_firmware=version,source_archive=name+'-sources.zip',
                    storage='TF1 BASEOS root; finish first-boot expansion before adding games')
    write(stage/'System/slot-sp-build.json',json.dumps(manifest,indent=2)+'\n')
    (stage/'payload.sha256').unlink(missing_ok=True);(stage/'system.sha256').unlink(missing_ok=True)
    files=sorted(p for p in stage.rglob('*') if p.is_file())
    write(stage/'system.sha256',''.join(f'{sha(p)}  {p.relative_to(stage)}\n' for p in files if p.is_relative_to(stage/'System')))
    write(stage/'payload.sha256',''.join(f'{sha(p)}  {p.relative_to(stage)}\n' for p in files+[stage/'system.sha256']))
    payload=d/'payload.tar.gz'
    with tarfile.open(payload,'w:gz',compresslevel=6) as t:
        for p in sorted(stage.iterdir()):t.add(p,arcname=p.name)
    installer=d/'slot-sp-install'
    text=Path(info['installer_template']).read_text().replace('VERSION=slot-rg35xxsp-0.2.0',f'VERSION={name}').replace('RG35XXSP-START-HERE.txt',guide_name)
    write(installer,text)
    rcs=d/'rcS';dump(root,'/etc/init.d/rcS',rcs)
    text=rcs.read_text().replace('/dev/mmcblk1p1 /dev/mmcblk0p7','/dev/mmcblk0p7 /dev/mmcblk1p1')
    anchor='# Older rootfs images may lack the alias baked into current builds.'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''# Install only after TF1 expansion has completed.
if [ "$USB_STORAGE_MODE" -eq 0 ] && grep -q '^/dev/mmcblk0p7 /mnt/sdcard ' /proc/mounts; then
    if ! /usr/sbin/slot-sp-install > /mnt/sdcard/slot-install.log 2>&1; then
        /usr/bin/baseos-splash --important -1 "SLOT INSTALL FAILED: CHECK SLOT-INSTALL.LOG" 2>/dev/null
        while :; do sleep 60; done
    fi
fi
'''+anchor)
    assert text.index('/usr/sbin/expand-storage')<text.index('/usr/sbin/slot-sp-install')
    write(rcs,text)
    front=d/'frontend-session';dump(root,'/usr/sbin/frontend-session',front)
    write(front,front.read_text().replace('/dev/mmcblk1p1 /dev/mmcblk0p7','/dev/mmcblk0p7 /dev/mmcblk1p1'))
    adds=[('/usr/share/slot-sp/payload.tar.gz',payload,False),('/usr/sbin/slot-sp-install',installer,True),
          ('/etc/init.d/rcS',rcs,True),('/usr/sbin/frontend-session',front,True),
          ('/lib/aarch64-linux-gnu/libgomp.so.1',checked(info['libgomp']),False)]
    commands=['mkdir /usr/share/slot-sp']
    for internal,p,executable in adds:
        if internal in ['/etc/init.d/rcS','/usr/sbin/frontend-session']:commands.append(f'rm {internal}')
        commands.extend([f'write {p} {internal}',f'set_inode_field {internal} mode 0100{755 if executable else 644}'])
    write(d/'root-commands','\n'.join(commands)+'\n');run('debugfs','-w','-f',str(d/'root-commands'),str(root))
    for internal,p,_ in adds:
        dest=d/'readback';dump(root,internal,dest);assert sha(dest)==sha(p),internal
    fscheck=run('e2fsck','-fn',str(root));write(d/'rootfs-check.log',fscheck)
    # Test the exact embedded installer using the actual BaseOS BusyBox runtime.
    sandbox=d/'test-root';sandbox.mkdir();run('debugfs','-R',f'rdump / {sandbox}',str(root))
    def install(folder,broken=False):
        env=dict(os.environ,SLOT_INSTALL_ROOT='/'+folder,SLOT_INSTALL_PAYLOAD='/broken.tar.gz' if broken else '/usr/share/slot-sp/payload.tar.gz')
        return subprocess.run(['chroot',str(sandbox),'/bin/sh','/usr/sbin/slot-sp-install'],env=env,capture_output=True,text=True)
    (sandbox/'clean').mkdir();r=install('clean')
    write(d/'clean-install.log',r.stdout+r.stderr)
    assert r.returncode==0,(r.stdout+r.stderr)[-2000:]
    marker=sandbox/'clean/System'/('.'+name);assert marker.exists()
    for f,h in manifest['cores'].items():assert sha(sandbox/'clean/System'/f)==h
    assert sha(sandbox/'clean/System/slot')==manifest['frontend_sha256']
    for category in ['Games','Labels','Saves','States']:
        for system in manifest['systems']:assert (sandbox/'clean'/category/system).is_dir()
    assets=list((stage/'System/PPSSPP').rglob('*'));assets=[p for p in assets if p.is_file()];assert len(assets)==189
    for p in assets:assert sha(sandbox/'clean/BIOS/PPSSPP'/p.relative_to(stage/'System/PPSSPP'))==sha(p)
    assert install('clean').returncode==0
    sentinel=sandbox/'retry/Games/N64/Local.z64';sentinel.parent.mkdir(parents=True);sentinel.write_bytes(b'original diagnostic placeholder')
    write(sandbox/'retry/.slot-sp-installing','')
    damaged=bytearray(payload.read_bytes());damaged[-8]^=255;(sandbox/'broken.tar.gz').write_bytes(damaged)
    assert install('retry',True).returncode!=0
    assert not (sandbox/'retry/System'/('.'+name)).exists()
    assert install('retry').returncode==0
    assert sentinel.read_bytes()==b'original diagnostic placeholder'
    write(d/'installer-validation.log','PASS clean install; eight platform folders; exact frontend and nine cores; 189 assets; idempotence; corruption rejected before completion; interrupted retry preserves game sentinel.\n')
    image=OUT/(name+'.img');shutil.copyfile(base,image)
    with image.open('r+b') as dst,root.open('rb') as src:dst.seek(start);shutil.copyfileobj(src,dst,4<<20)
    gpt=run('sgdisk','-v',str(image));assert 'No problems found' in gpt,gpt
    ranges_equal(image,base,[(0,start),(end,base.stat().st_size)])
    manifest.update(version=version,device=label,baseos_archive_sha256=info['base']['sha256'],
                    vendor_boot_regions_byte_identical=True,installer_embedded=True)
    validation=f'{label} {version}\nPASS official base archive SHA256; target identity; rootfs e2fsck; all added-file readbacks; real BusyBox installer and interrupted/corrupt install checks; exact frontend/nine cores/189 assets; GPT consistency; all non-rootfs regions byte-identical to official image; ZIP CRC and decompressed image SHA256.\nPreviously validated frontend rendering/control tests remain applicable because the installed executable is byte-identical. Physical boot of this fresh image is pending.\n'
    publish_local(model,version,image,manifest,guide,validation)
def g350():
    info=inputs['g350'];version=info['version'];name=f'slot-g350-{version}'
    d=W/'g350';d.mkdir(parents=True,exist_ok=True)
    base=d/'previous-base.img'
    with zipfile.ZipFile(checked(info['base'])) as z:
        names=[n for n in z.namelist() if n.endswith('.img')];assert len(names)==1
        with z.open(names[0]) as src,base.open('wb') as dst:shutil.copyfileobj(src,dst,4<<20)
    system=d/'SYSTEM'
    with zipfile.ZipFile(checked(info['update'])) as z:
        with z.open('SYSTEM') as src,system.open('wb') as dst:shutil.copyfileobj(src,dst,4<<20)
        (d/'SYSTEM.md5').write_bytes(z.read('SYSTEM.md5'))
        if 'KERNEL' in z.namelist():
            (d/'KERNEL').write_bytes(z.read('KERNEL'))
            (d/'KERNEL.md5').write_bytes(z.read('KERNEL.md5'))
    manifest=json.loads(Path(info['system_manifest']).read_text())
    assert sha(system)==manifest['system_sha256']
    assert (d/'SYSTEM.md5').read_text().strip()==sha(system,'md5')+'  target/SYSTEM'
    image=OUT/(name+'.img');shutil.copyfile(base,image)
    fat=f'{image}@@16777216'
    run('mdel','-i',fat,'::SYSTEM');run('mdel','-i',fat,'::SYSTEM.md5')
    run('mcopy','-o','-i',fat,str(system),'::SYSTEM');run('mcopy','-o','-i',fat,str(d/'SYSTEM.md5'),'::SYSTEM.md5')
    if (d/'KERNEL').exists():
        assert sha(d/'KERNEL')==manifest['kernel_sha256']
        assert (d/'KERNEL.md5').read_text().strip()==sha(d/'KERNEL','md5')+'  target/KERNEL'
        run('mcopy','-o','-i',fat,str(d/'KERNEL'),'::KERNEL')
        run('mcopy','-o','-i',fat,str(d/'KERNEL.md5'),'::KERNEL.md5')
        run('mcopy','-i',fat,'::KERNEL',str(d/'KERNEL-readback'))
        assert sha(d/'KERNEL-readback')==manifest['kernel_sha256']
    zeroed=zero_free_fat_clusters(image,16777216)
    run('mcopy','-i',fat,'::SYSTEM',str(d/'SYSTEM-readback'))
    assert sha(d/'SYSTEM-readback')==manifest['system_sha256']
    run('mcopy','-i',fat,'::SYSTEM.md5',str(d/'SYSTEM.md5-readback'))
    assert (d/'SYSTEM.md5-readback').read_bytes()==(d/'SYSTEM.md5').read_bytes()
    storage_start=4227072*512;storage_end=storage_start+65536*512
    ranges_equal(image,base,[(0,16777216),(storage_start,storage_end)])
    fatcheck=d/'fat.img';storage=d/'storage.ext4'
    with image.open('rb') as src:
        for p,start,size in [(fatcheck,16777216,4194304*512),(storage,storage_start,65536*512)]:
            with p.open('wb') as dst:
                src.seek(start);left=size
                while left:
                    b=src.read(min(4<<20,left));assert b;dst.write(b);left-=len(b)
    write(d/'filesystem-validation.log',run('fsck.fat','-n',str(fatcheck))+run('e2fsck','-fn',str(storage)))
    assert '.please_resize_me' in run('debugfs','-R','ls -p /',str(storage))
    guide=f'''Slot BATLEXP G350 {version} - fresh installation

Extract slot-g350-{version}.img.zip and flash its .img to a 16 GB or larger
SD card. Flashing erases that card, so back up games/labels/saves first.
Use the card in the G350 OS/TF1 slot. This is the G350 build, not an H700 build.
Let the first boot expand the empty data partition and finish its restart.
After reaching Slot, shut down before adding games. No older SYSTEM update
is needed: this image already contains the current {version} SYSTEM and KERNEL pairs, including the gray SLOT splash.

On GAMES, put games under Slot/Games/GB, GBC, GBA, SNES, PS1, N64, DC or PSP.
Native Ports are integrated, and PortMaster installs automatically on first
boot. Open it from the Ports shelf; downloads need internet. User game files
and extra runtimes remain separate. Ports artwork uses Slot/Labels/PORTS.
Labels go in matching Slot/Labels folders, with matching filename stems.
Restore Slot/BIOS, Saves, States, Config and Wallpapers if you have a backup.
No games or proprietary BIOS are included. PPSSPP's free assets are bundled.
The repository's studio/ directory contains Artwork Studio v5.

Hold Menu alone one second to return to Slot. Console states: Menu+X saves,
Menu+Y loads. GB/GBC/GBA: Select+R1 saves; Select+L1 loads (prompt chord).
GB/GBC: L1 cycles stretch/full-height aspect fit, R1 restores default.
Tap Menu at the shelf for Shader: Original, Sharp Pixels, Smooth, LCD, CRT.
See the repository README for all controls and emulator-specific hotkeys.

The original dedicated PAN4ELEC G350 bootloader/default panel files and
first-boot resize setup are preserved. The matching latest SYSTEM and MD5
are installed in the image and read back for verification. The G350 {version}
frontend/system update was validated separately; physical first boot of this
newly composed fresh image is pending. No new performance claim is made.
'''
    validation='PASS verified old fresh-image input and latest SYSTEM input; SYSTEM/MD5 image readbacks; FAT/ext4 checks; original bootloader and empty storage partition unchanged; first-boot resize marker retained; ZIP CRC/decompressed image SHA256. Includes the accepted 0.1.13 frontend/cores and the bundled-manager bootstrap. Fresh physical first boot remains pending.\n'
    manifest.update(version=version,device='BATLEXP G350',base_fresh_image_sha256=info['base']['sha256'],bootloader_and_initial_storage_unchanged=True,free_boot_clusters_zeroed_bytes=zeroed)
    publish_local('g350',version,image,manifest,guide,validation)
if args.model in ['g350','all']:g350()
for model in ['rg35xxsp','rgsp']:
    if args.model in [model,'all']:h700(model)
