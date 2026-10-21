from pathlib import Path
import tarfile,subprocess,hashlib,json,shutil
import argparse
p=argparse.ArgumentParser()
p.add_argument('cache',type=Path);p.add_argument('output',type=Path);p.add_argument('--build',type=Path,default=Path('/tmp/slot-sp-native-n64'))
args=p.parse_args()
cache=args.cache.resolve();w=args.output.resolve();root=args.build.resolve();w.mkdir(parents=True,exist_ok=True)
root.mkdir(exist_ok=True);out=w/'native';(out/'plugins').mkdir(parents=True,exist_ok=True)
meta=json.loads(Path(__file__).with_name('source-references.json').read_text())
sources={}
for record in meta:
    name=record['component'];archive=cache/('mupen64plus-'+name+'-'+record['upstream_revision']+'.tar.gz')
    assert hashlib.sha256(archive.read_bytes()).hexdigest()==record['source_sha256']
    dest=root/name
    if not dest.exists():
        dest.mkdir()
        with tarfile.open(archive) as z:
            for m in z.getmembers():
                rel=Path(*Path(m.name).parts[1:]);assert '..' not in rel.parts
                if m.isfile():
                    p=dest/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(z.extractfile(m).read())
    sources[name]=dest
env=dict(__import__('os').environ)
flags='-O3 -mcpu=cortex-a53 -fPIC'
patch=Path(__file__).with_name('rice-centered.patch')
if not (sources['video-rice']/'.slot-patched').exists():
    subprocess.run(['patch','-p1','-i',str(patch)],cwd=sources['video-rice'],check=True)
    (sources['video-rice']/'.slot-patched').touch()
for name in ['core','audio-sdl','input-sdl','rsp-hle','video-rice']:
    args=['make','-C',str(sources[name]/'projects/unix'),'-j4','all',
        'HOST_CPU=aarch64','APIDIR='+str(sources['core']/'src/api'),
        'OPTFLAGS='+flags,'V=1']
    if name=='core':args+=['NEON=1','DYNAREC=arm64','NEW_DYNAREC=1','OSD=0','VULKAN=0','USE_GLES=1']
    if name=='video-rice':args+=['USE_GLES=1']
    subprocess.run(args,check=True,env=env)
    binary=next((sources[name]/'projects/unix').glob('libmupen64plus.so*')) if name=='core' else sources[name]/'projects/unix'/('mupen64plus-'+name+'.so')
    destination=out/'libmupen64plus.so.2' if name=='core' else out/'plugins'/binary.name
    shutil.copyfile(binary,destination)
    subprocess.run(['strip','--strip-unneeded',str(destination)],check=True)
for name in ['core','video-rice']:
    for p in (sources[name]/'data').glob('*.ini'):shutil.copyfile(p,out/p.name)
(out/'source-references.json').write_text(json.dumps(meta,indent=2))
print('PASS built Cortex-A53 native Mupen64Plus/Rice/HLE/SDL components')
