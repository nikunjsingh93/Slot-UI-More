from pathlib import Path
import hashlib,json,shutil,zipfile,subprocess
w=Path('/tmp/slot-release-116');repo=Path('/work/github-g350');out=Path('/outputs/release-1.1.16');out.mkdir(exist_ok=True)
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(4<<20),b''):h.update(b)
 return h.hexdigest()
common=w/'h700-common';common.mkdir(exist_ok=True)
shutil.copytree('/work/rg35xxsp-v0100/payload/System',common/'System',dirs_exist_ok=True)
# Start from a previously distributed clean runtime, never an owner's card.
with zipfile.ZipFile('/outputs/slot-rgsp-1.1.15-system-files.zip') as z:
 for item in z.infolist():
  rel=Path(item.filename)
  assert not rel.is_absolute() and '..' not in rel.parts
  if not item.is_dir() and rel.parts[0]=='System':
   p=common/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(z.read(item))
# Only approved emulator/runtime deltas; no commercial port trees or user data.
for folder in ['rgsp-fixes-20261020','rgsp-followup-20261020','rgsp-036']:
 shutil.copytree(Path('/work')/folder/'payload/System',common/'System',dirs_exist_ok=True)
sys=common/'System'
shutil.copyfile('/tmp/slot-sp-036/target/device/slot',sys/'slot')
for p in (w/'helpers').iterdir():shutil.copyfile(p,sys/p.name)
for name in ['slot-h700-port-runner.sh','slot-refresh-ports.py','slot-h700-native-gl.sh']:
 shutil.copyfile(repo/'firmware/ports'/name,sys/name)
shutil.copyfile(repo/'firmware/ports/slot-native-n64.py',sys/'N64/slot-native-n64.py')
templates=sys/'PortTemplates';templates.mkdir(exist_ok=True)
for p in (repo/'ports/h700/launchers').glob('*.sh'):
 if 'Most Wanted' not in p.name:shutil.copyfile(p,templates/p.name)
shutil.copyfile(repo/'ports/h700/ADDITIONAL-PORTS.md',templates/'README.md')
shutil.copytree(w/'stk-compat',sys/'PortCompat/supertuxkart',dirs_exist_ok=True)
for name in ['slot-no-grab.armhf.so','slot-h700-opaque.so','slot-h700-gta-postfx.so','ADDED-PORTS.txt']:(sys/name).unlink(missing_ok=True)
# Remove stale prior completion markers from the clean packaging baseline.
for p in sys.glob('.slot-*'):p.unlink()
accepted=json.loads(Path('/outputs/slot-g350-1.1.15-manifest.json').read_text())
for name,digest in accepted['core_sha256'].items():assert sha(sys/name)==digest,name
for p in sys.rglob('*'):
 if p.is_file():
  with p.open('rb') as f:magic=f.read(4)
  p.chmod(0o755 if magic.startswith((b'#!',b'\x7fELF')) else 0o644)
inputs={}
base=json.loads(Path('/work/latest-releases/inputs.json').read_text())
for model,label in [('rgsp','Anbernic RG SP'),('rg35xxsp','Anbernic RG35XX SP')]:
 d=w/(model+'-overlay');shutil.copytree(common,d)
 shutil.copyfile(repo/'ports'/model/'launch_frontend.sh',d/'System/launch_frontend.sh')
 m=json.loads((sys/'slot-sp-build.json').read_text())
 m.update(version='1.1.16',device=label,experimental=False,hardware_tested=False,frontend_sha256=sha(sys/'slot'),
  base_firmware='1.1.16',source_archive='slot-1.1.16-sources.zip',
  baseos_archive_sha256=base[model]['base']['sha256'],panel='720x480 (3:2)' if model=='rgsp' else '640x480 (4:3)',
  console_aspect='Per-model default proportions; per-system Full option',native_n64_hardware_tested=model=='rgsp',
  native_ports_hardware_tested=model=='rgsp',runtime_hardware_tested=model=='rgsp',
  physical_validation='RG SP predecessor runtime owner-tested; this newly packaged image requires first-boot validation. RG35XX SP transferred changes not hardware-tested.',
  gta_presentation='Original effects; GTA-only SDL_BLITTER_DISABLED=1 direct EGL, owner-confirmed on RG SP',
  included_port_templates=['GTA III','Vice City','GTA SA','NFS Underground 2','RVGL','SuperTuxKart'],
  removed_ports=['Most Wanted Mobile'],games_included=False,proprietary_bios_included=False)
 m['native_components']={p.relative_to(d/'System').as_posix():sha(p) for p in (d/'System/N64').rglob('*') if p.is_file()}
 (d/'System/slot-sp-build.json').write_text(json.dumps(m,indent=2)+'\n')
 info=base[model];info.update(version='1.1.16',frontend_payload=str(d),installer_template=str(repo/'ports/rg35xxsp/slot-sp-install'))
 inputs[model]=info
# G350 retains its owner-accepted SYSTEM/kernel exactly; refresh complete image.
gm=json.loads(Path('/outputs/slot-g350-1.1.15-manifest.json').read_text())
gm.update(version='1.1.16',image_revision=1,release_note='Accepted G350 runtime preserved; synchronized complete-image release 1.1.16.')
(w/'g350-system-manifest.json').write_text(json.dumps(gm,indent=2)+'\n')
inputs['g350']={'version':'1.1.16','base':{'path':'/outputs/slot-g350-1.1.15.img.zip','sha256':sha(Path('/outputs/slot-g350-1.1.15.img.zip'))},
 'update':{'path':'/outputs/slot-g350-1.1.15-system-files.zip','sha256':sha(Path('/outputs/slot-g350-1.1.15-system-files.zip'))},'system_manifest':str(w/'g350-system-manifest.json')}
(w/'image-inputs.json').write_text(json.dumps(inputs,indent=2)+'\n')
print('PASS staged model-specific clean H700 overlays and pinned accepted G350 inputs',flush=True)
