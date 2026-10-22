"""Build pinned NES/Sega/PCE/arcade cores in the ARM64 Bullseye builder.

python3 scripts/build_extra_cores.py --work /tmp/slot-extra-build --cache /path/to/archives
No game data is needed. Build outputs and full corresponding source archives stay
outside Git. The same ARMv8-A software cores serve RK3326 and H700.
"""
import argparse, hashlib, json, shutil, subprocess, tarfile, urllib.request
from pathlib import Path

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work',type=Path,required=True)
    parser.add_argument('--cache',type=Path,required=True)
    parser.add_argument('--jobs',type=int,default=4)
    args=parser.parse_args()
    repo=Path(__file__).resolve().parents[1]
    pins=json.loads((repo/'docs/additional-core-sources.json').read_text())
    args.cache.mkdir(parents=True,exist_ok=True)
    output=args.work/'cores';output.mkdir(parents=True,exist_ok=True)
    result=[]
    for pin in pins:
        archive=args.cache/pin['archive']
        if not archive.exists():
            with urllib.request.urlopen(pin['url'],timeout=180) as src,archive.open('wb') as dst:
                shutil.copyfileobj(src,dst)
        assert hashlib.sha256(archive.read_bytes()).hexdigest()==pin['sha256'],archive
        source=args.work/'source'/pin['name']
        if not source.exists():
            source.mkdir(parents=True)
            # Verified official archives; remove the single enclosing directory.
            with tarfile.open(archive) as tar:
                for entry in tar.getmembers():
                    entry.name=entry.name.partition('/')[2]
                    if entry.name:
                        if Path(entry.name).is_absolute() or '..' in Path(entry.name).parts:
                            raise ValueError('Unsafe source archive member')
                        tar.extract(entry,source)
        build=source/pin['build_subdir']
        subprocess.run(['make','-f',pin['makefile'],f'-j{args.jobs}','platform=unix'],cwd=build,check=True)
        binary=output/(pin['name']+'_libretro.so')
        shutil.copyfile(build/binary.name,binary)
        subprocess.run(['strip','--strip-unneeded',str(binary)],check=True)
        notices=output/'notices'/pin['name'];notices.mkdir(parents=True,exist_ok=True)
        # FBNeo's root LICENSE.md only links its actual license in src/.
        for name in ['COPYING','Copying','COPYING.txt','LICENSE','LICENSE.txt','LICENSE.md','src/license.txt']:
            path=source/name
            if path.is_file():shutil.copyfile(path,notices/path.name)
        shutil.copyfile(archive,notices/archive.name)
        record={**pin,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()}
        (notices/'source.json').write_text(json.dumps(record,indent=2)+'\n')
        if pin['name']=='fbneo':
            shutil.copyfile(source/'dats/FinalBurn Neo (ClrMame Pro XML, Arcade only).dat',output/'FBNeo-Arcade.dat')
        result.append(record)
    (output/'manifest.json').write_text(json.dumps(result,indent=2)+'\n')

if __name__=='__main__':main()
