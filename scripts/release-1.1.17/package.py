"""Package current Git source and checked source archives after full image checks."""
from pathlib import Path
import hashlib,json,shutil,subprocess,zipfile
repo=Path('/work/github-g350');out=Path('/outputs/release-1.1.17');work=Path('/tmp/slot-release-117')
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(4<<20),b''):h.update(b)
    return h.hexdigest()
commit=subprocess.check_output(['git','-C',str(repo),'rev-parse','HEAD'],text=True).strip()
source=out/'slot-1.1.17-sources.zip'
subprocess.run(['git','-C',str(repo),'archive','--format=zip','--output='+str(source),commit],check=True)
native=repo/'ports/h700/n64-native';pins=json.loads((native/'source-references.json').read_text())
with zipfile.ZipFile(source,'a',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
    for pin in pins:
        name='mupen64plus-'+pin['component']+'-'+pin['upstream_revision']+'.tar.gz'
        p=Path('/work/g350/native-upstream')/name;assert sha(p)==pin['source_sha256']
        z.write(p,'vendor-sources/native-n64/'+name,compress_type=zipfile.ZIP_STORED)
    z.write(Path('/tmp/slot-release-116/h700-common/System/N64/muos-reference.cfg'),'vendor-sources/native-n64/muos-reference.cfg')
    z.writestr('RELEASE-SOURCE-COMMIT.txt',commit+'\nFull G350 and H700 frontends/build/install scripts included.\nNew core sources: slot-1.1.17-additional-core-sources.zip.\nUnchanged OS/core source provenance: retained 1.1.15 releases and docs/SOURCE-REVISIONS.txt.\n')
additional=out/'slot-1.1.17-additional-core-sources.zip';cores=Path('/tmp/slot-extra/cores')
with zipfile.ZipFile(additional,'w',zipfile.ZIP_DEFLATED,compresslevel=3) as z:
    for pin in json.loads((cores/'manifest.json').read_text()):
        p=cores/'notices'/pin['name']/pin['archive'];assert sha(p)==pin['sha256'];z.write(p,p.name)
    for p,n in [(repo/'docs/additional-core-sources.json','sources.json'),(repo/'scripts/build_extra_cores.py','build_extra_cores.py')]:z.write(p,n)
compat=out/'slot-1.1.17-compatibility-sources.zip'
shutil.copyfile('/outputs/release-1.1.16/slot-1.1.16-compatibility-sources.zip',compat)
shutil.copyfile(repo/'studio/Slot-Auto-Artwork-Studio-v5.html',out/'Slot-Auto-Artwork-Studio-v5.html')
for p in [source,additional,compat]:
    with zipfile.ZipFile(p) as z:assert z.testzip() is None,p
assets={'version':'1.1.17','source_commit':commit,'models':{}}
validation={'version':'1.1.17','targeted_software_tests':46,'physical_first_boot_tested':False,
            'new_system_games_hardware_tested':False,'models':{}}
for model in ['g350','rgsp','rg35xxsp']:
    prefix=f'slot-{model}-1.1.17';fresh=out/(prefix+'-fresh-manifest.json');manifest=out/(prefix+'-manifest.json')
    assert fresh.exists();fresh.rename(manifest)
    m=json.loads(manifest.read_text());assert m['version']=='1.1.17'
    files=[out/(prefix+s) for s in ['.img.zip','-manifest.json','-START-HERE.txt','-VALIDATION.txt']]
    if model=='g350':files += [source,additional,compat,out/'Slot-Auto-Artwork-Studio-v5.html']
    sums=out/(prefix+'-SHA256SUMS.txt');sums.write_text(''.join(sha(p)+'  '+p.name+'\n' for p in files));files.append(sums)
    assets['models'][model]={'release_tag':model+'-v1.1.17','assets':{p.name:{'size':p.stat().st_size,'sha256':sha(p)} for p in files}}
    validation['models'][model]={'image_sha256':m['image_sha256'],'zip_sha256':m['zip_sha256'],
                                 'frontend_sha256':m['frontend_sha256'],'zip_bytes':m['zip_bytes']}
    shutil.copyfile(manifest,repo/'docs/releases'/manifest.name)
(repo/'docs/releases/assets-1.1.17.json').write_text(json.dumps(assets,indent=2)+'\n')
(repo/'docs/releases/validation-1.1.17.json').write_text(json.dumps(validation,indent=2)+'\n')
downloads=json.loads((repo/'docs/downloads.json').read_text())
for model,info in assets['models'].items():
    for name,record in info['assets'].items():downloads[name]={'sha256':record['sha256'],'size':record['size'],'tag':info['release_tag']}
(repo/'docs/downloads.json').write_text(json.dumps(downloads,indent=2)+'\n')
(work/'assets.json').write_text(json.dumps(assets,indent=2)+'\n')
print('PASS release source snapshot, corresponding sources and per-model asset inventories',flush=True)
