"""Build compact SP input test updates; never access a card or include games.
Run inside the ARM64 Bullseye builder with gcc and libasound development files.
"""
from pathlib import Path
import argparse,hashlib,json,subprocess,tempfile,zipfile
p=argparse.ArgumentParser()
p.add_argument('--outputs',type=Path,required=True)
args=p.parse_args();out=args.outputs;out.mkdir(parents=True,exist_ok=True)
repo=Path(__file__).resolve().parent.parent;ports=repo/'firmware/ports'
def sha(b):return hashlib.sha256(b).hexdigest()
with tempfile.TemporaryDirectory(prefix='slot-sp-input-package-') as tmp:
 binary=Path(tmp)/'slot-portwatch'
 subprocess.run(['gcc','-O2','-Wall','-Wextra','-Werror',str(ports/'portwatch.c'),'-o',str(binary),'-lasound','-lm'],check=True)
 subprocess.run(['strip','--strip-unneeded',str(binary)],check=True)
 for model,label in [('rgsp','RG SP'),('rg35xxsp','RG35XX SP')]:
  name=f'slot-{model}-1.1.15-native-input-test'
  files={'System/slot-portwatch':binary.read_bytes(),
         'System/launch_frontend.sh':(repo/'ports'/model/'launch_frontend.sh').read_bytes()}
  guide=f'''Slot {label} native-port input TEST update, based on 1.1.15

Only for an existing matching Slot {label} card with PortMaster installed.
This is NOT a flashable image. Do not format or reflash the card.
With the handheld off, back up System/slot-portwatch and
System/launch_frontend.sh to your computer. Merge this ZIP's System folder
into the BASEOS root, replacing those TWO files. Leave everything else intact.
Safely eject and boot. The other model's launcher is incompatible.

In a native port, hold Menu then press Select to toggle D-pad / left stick.
The mode starts as D-pad for ordinary ports, analog for the named GTA SA/NFS
launchers, and is remembered separately for each .sh launcher.
Hold Menu ALONE for one second to quit. Release both buttons after toggling.
Normal Select remains available when Menu is not held. Analog is full tilt.
The PortMaster manager remains in normal D-pad mode. Emulator controls and
save-state shortcuts are unchanged. Named GTA SA/NFS U2 controller profiles
are included; game runtimes and data are supplied separately.

Modes are stored in Config/port-input/<launcher>.sh.mode (0=dpad, 1=analog).
Remove one game's mode file while powered off to reset just that game.
Names follow the launcher: renaming it gives that game a new default mode.
Logs: Config/ports.log; look for 'native pad' and 'input mode='.

Tests passed: both SP mode mappings, diagonals/releases, held-key switching,
per-game persistence/isolation, chord repeat/suppression, Menu-only quit,
volume/mute/headphone regressions, actual isolated device-path routing.
NOT YET TESTED on a physical handheld: uinput/namespace availability,
native games, graphics/audio and real controller enumeration.
Requires the firmware kernel's uinput and mount-namespace support. Setup
failure returns to Slot; it does not change boot files or game data.
Keyboard-style ports still depend on the port's own mappings for analog mode.
There are no mouse/right-stick modes or game binary patches in this test.
For Need for Speed Underground 2.sh, Menu+X opens its native R3 keyboard.
The physical SP Select key is code 310; code 314 remains L2.

To undo, power off and restore the two original System files. Keep your
Games, Labels, BIOS, Saves, States, Config and port game directories.
Do not use this update on G350. Stable full images have NOT been replaced.
'''
  files['NATIVE-INPUT-TEST.txt']=guide.encode()
  metadata={'model':model,'base_version':'1.1.15','status':'test update; physical validation pending',
            'kernel_requirements':['uinput','mount namespaces'],
            'files':{n:sha(b) for n,b in files.items()},
            'source_sha256':{n:sha((ports/n).read_bytes()) for n in ['portwatch.c','port-input.h','port-volume.h']},
            'games_included':False,
            'gta_binary_patch':False,'named_game_profiles':['Grand Theft Auto San Andreas.sh','Need for Speed Underground 2.sh'],'emulator_files_changed':False}
  files['native-input-test.json']=(json.dumps(metadata,indent=2)+'\n').encode()
  files['SHA256SUMS.txt']=''.join(sha(b)+'  '+n+'\n' for n,b in files.items()).encode()
  archive=out/(name+'.zip')
  with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as z:
   for n,b in files.items():
    info=zipfile.ZipInfo(n);info.compress_type=zipfile.ZIP_DEFLATED
    info.external_attr=(0o100755 if n.startswith('System/') else 0o100644)<<16
    z.writestr(info,b)
  with zipfile.ZipFile(archive) as z:
   assert z.testzip() is None
   assert set(n for n in z.namelist() if n.startswith('System/'))=={'System/slot-portwatch','System/launch_frontend.sh'}
   for n,b in files.items():assert z.read(n)==b
  (out/(name+'-SHA256.txt')).write_text(sha(archive.read_bytes())+'  '+archive.name+'\n')
  print('PASS packaged '+archive.name,flush=True)
 sources=out/'slot-sp-native-input-test-sources.zip'
 with zipfile.ZipFile(sources,'w',zipfile.ZIP_DEFLATED) as z:
  for n in ['portwatch.c','port-input.h','port-volume.h','test_port_input.py','test_port_input_namespace.py','test_portwatch.py','test_port_volume.py']:
   z.write(ports/n,'firmware/ports/'+n)
  for model in ['rgsp','rg35xxsp']:z.write(repo/'ports'/model/'launch_frontend.sh','ports/'+model+'/launch_frontend.sh')
  z.write(Path(__file__),'scripts/package_sp_native_input.py');z.write(repo/'LICENSE','LICENSE')
  z.write(repo/'ports/h700/native-input-validation.json','ports/h700/native-input-validation.json')
  z.write(repo/'ports/h700/README.md','ports/h700/README.md')
  for f in (repo/'ports/h700/launchers').glob('*.sh'):z.write(f,'ports/h700/launchers/'+f.name)
 print('PASS corresponding input source and test bundle',flush=True)
