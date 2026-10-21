from pathlib import Path
import hashlib,json,subprocess,zipfile,shutil
w=Path('/tmp/slot-release-116');r=Path('/work/github-g350');out=Path('/outputs/release-1.1.16')
commit=subprocess.check_output(['git','-C',str(r),'rev-parse','HEAD'],text=True).strip()
source=out/'slot-1.1.16-sources.zip'
subprocess.run(['git','-C',str(r),'archive','--format=zip','--output='+str(source),commit],check=True)
native=r/'ports/h700/n64-native';records=json.loads((native/'source-references.json').read_text())
with zipfile.ZipFile(source,'a',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
 for item in records:
  name='mupen64plus-'+item['component']+'-'+item['upstream_revision']+'.tar.gz';p=Path('/work/g350/native-upstream')/name
  assert hashlib.sha256(p.read_bytes()).hexdigest()==item['source_sha256'],p
  z.write(p,'vendor-sources/native-n64/'+name,compress_type=zipfile.ZIP_STORED)
 z.write(w/'h700-common/System/N64/muos-reference.cfg','vendor-sources/native-n64/muos-reference.cfg')
 z.write(native/'source-references.json','vendor-sources/native-n64/source-references.json')
 z.writestr('RELEASE-SOURCE-COMMIT.txt',commit+'\nFull G350 frontend: slot/\nCommon H700 frontend: ports/h700/frontend/\nExact native N64 source archives: vendor-sources/native-n64/\nHistorical unchanged core source archives: g350-v1.1.15 release\n')
for model in ['rgsp','rg35xxsp']:shutil.copyfile(source,out/f'slot-{model}-1.1.16-sources.zip')
compat=out/'slot-1.1.16-compatibility-sources.zip'
with zipfile.ZipFile(compat,'w',zipfile.ZIP_STORED) as z:
 for p in sorted((w/'compat-sources').iterdir()):
  if p.is_file():z.write(p,'debian-source-packages/'+p.name)
 z.write(w/'stk-compat/sources.json','sources.json')
 for p in (w/'stk-compat/licenses').rglob('*'):
  if p.is_file():z.write(p,p.relative_to(w/'stk-compat').as_posix())
for p in [source,compat]:
 with zipfile.ZipFile(p) as z:assert z.testzip() is None,p
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(4<<20),b''):h.update(b)
 return h.hexdigest()
for model in ['g350','rgsp','rg35xxsp']:
 prefix=f'slot-{model}-1.1.16'
 files=[out/(prefix+s) for s in ['.img.zip','-manifest.json','-START-HERE.txt','-VALIDATION.txt']]
 files.append(source if model=='g350' else out/(prefix+'-sources.zip'))
 if model=='g350':files.append(compat)
 sums=out/(prefix+'-SHA256SUMS.txt');sums.write_text(''.join(sha(p)+'  '+p.name+'\n' for p in files));files.append(sums)
 (w/(model+'-assets.json')).write_text(json.dumps({p.name:{'size':p.stat().st_size,'sha256':sha(p)} for p in files},indent=2)+'\n')
print('PASS full release sources, five native N64 source archives, twenty exact Debian source packages and per-model checksums.',flush=True)
