from pathlib import Path
import hashlib,json,shutil,subprocess
W=Path('/work/rgsp-migration')
def run(*args):
    r=subprocess.run(args,check=True,capture_output=True,text=True)
    return r.stdout
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(4<<20),b''):h.update(b)
    return h.hexdigest()
def write(p,s):
    with p.open('w',encoding='utf-8',newline='\n') as f:f.write(s)
root=W/'rootfs-rgsp.ext4'
shutil.copyfile(W/'rootfs-original.ext4',root)
add=[]
for internal,name in [('/etc/init.d/rcS','rcS-rgsp'),('/usr/sbin/frontend-session','frontend-session-rgsp')]:
    p=W/name
    run('debugfs','-R',f'dump {internal} {p}',str(root))
    write(p,p.read_text().replace('/dev/mmcblk1p1 /dev/mmcblk0p7','/dev/mmcblk0p7 /dev/mmcblk1p1'))
    add.append((internal,p,True))
add.append(('/lib/aarch64-linux-gnu/libgomp.so.1',Path('/tmp/rg35xxsp-root/lib/aarch64-linux-gnu/libgomp.so.1').resolve(),False))
cmd=[]
for internal,p,executable in add:
    if executable:cmd.append(f'rm {internal}')
    cmd.extend([f'write {p} {internal}',f'set_inode_field {internal} mode 0100{755 if executable else 644}'])
write(W/'root-commands','\n'.join(cmd)+'\n')
run('debugfs','-w','-f',str(W/'root-commands'),str(root))
for internal,p,_ in add:
    dest=W/'root-readback'; dest.unlink(missing_ok=True)
    run('debugfs','-R',f'dump {internal} {dest}',str(root))
    assert sha(dest)==sha(p),internal
write(W/'rootfs-validation.log',run('e2fsck','-fn',str(root)))
assert 'BASEOS_TARGET=rgsp' in run('debugfs','-R','cat /etc/baseos-release',str(root))
# This conversion never writes GPT, the inactive slot, persistent userdata or
# the expanded FAT partition. Only the audited H700 boot chain and active p5.
boot=W/'rgsp-boot-region.bin'
with (W/'baseos-rgsp.img').open('rb') as src,boot.open('wb') as dst:
    # Allwinner boot0 starts at 8 KiB and differs between these two models.
    # Preserve the complete GPT below it, but replace boot0 as a whole.
    src.seek(8192); remaining=222298112-8192
    while remaining:
        b=src.read(min(4<<20,remaining));assert b;dst.write(b);remaining-=len(b)
regions=[dict(path=boot.name,offset=8192,size=boot.stat().st_size,sha256=sha(boot)),
         dict(path=root.name,offset=222298112,size=root.stat().st_size,sha256=sha(root))]
write(W/'boot-regions.json',json.dumps(regions,indent=2)+'\n')
print('PASS model-specific RG SP boot region, p5 filesystem and added-file readbacks',flush=True)
