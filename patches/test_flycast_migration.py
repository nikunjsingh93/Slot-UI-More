from pathlib import Path
import hashlib, os, subprocess, tempfile, shutil
root=Path('/tmp/g350-v019-root')
s=(root/'usr/bin/slot-start').read_text()
block=s.split('# Replace the known 0.1.9/0.1.9.1 Flycast core, retaining custom cores.\n')[1].split('# Replace only known bundled')[0]
block=block.replace('/usr/share/slot/flycast_libretro.so',str(root/'usr/share/slot/flycast_libretro.so'))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
with tempfile.TemporaryDirectory(prefix='g350-migration-') as d:
    task_root=Path(d);(task_root/'System').mkdir();target=task_root/'System/flycast_libretro.so'
    shutil.copy2('/tmp/flycast-v0191-original.so',target)
    env=os.environ.copy();env['SLOT_ROOT']=str(task_root)
    subprocess.run(['sh','-ec',block],env=env,check=True)
    assert digest(target)==digest(root/'usr/share/slot/flycast_libretro.so')
    subprocess.run(['sh','-ec',block],env=env,check=True)
    assert digest(target)==digest(root/'usr/share/slot/flycast_libretro.so')
    target.write_bytes(b'owner custom core')
    subprocess.run(['sh','-ec',block],env=env,check=True)
    assert target.read_bytes()==b'owner custom core'
    assert not target.with_suffix('.so.tmp').exists()
print('PASS old bundled Flycast upgraded, second boot idempotent, custom core preserved')
