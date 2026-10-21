"""Record provenance and notices for the game-local Debian Bullseye STK fixes.

Run in the same ARM64 Bullseye builder that supplied the libraries. Download
the listed exact source packages with apt-get source --download-only before
publishing the resulting compatibility binaries.
"""
from pathlib import Path
import argparse,hashlib,json,shutil,subprocess
p=argparse.ArgumentParser();p.add_argument('libraries',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
out=a.output;out.mkdir(parents=True,exist_ok=True);records=[];sources={}
for file in sorted(a.libraries.iterdir()):
 if not file.is_file():continue
 candidates=[Path('/usr/lib/aarch64-linux-gnu')/file.name,Path('/lib/aarch64-linux-gnu')/file.name]
 original=next(x for x in candidates if x.exists()).resolve()
 digest=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
 assert digest(file)==digest(original),file
 owner=subprocess.check_output(['dpkg-query','-S',str(original)],text=True).split(': /',1)[0]
 metadata=subprocess.check_output(['dpkg-query','-W','-f=${source:Package}\t${source:Version}\t${Version}',owner],text=True).split('\t')
 source,version,binary_version=metadata;sources[source]=version
 target=out/'lib'/file.name;target.parent.mkdir(exist_ok=True);shutil.copyfile(file,target)
 notice=Path('/usr/share/doc')/owner.split(':')[0]/'copyright'
 assert notice.is_file(),notice
 dest=out/'licenses'/owner.split(':')[0];dest.mkdir(parents=True,exist_ok=True);shutil.copyfile(notice,dest/'copyright')
 records.append({'file':file.name,'sha256':digest(file),'binary_package':owner,'binary_version':binary_version,'source_package':source,'source_version':version})
(out/'sources.json').write_text(json.dumps({'distribution':'Debian Bullseye ARM64','libraries':records,'source_packages':sources},indent=2)+'\n')
print(json.dumps(sources,indent=2))
