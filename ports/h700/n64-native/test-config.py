"""Check native N64 panel/config behavior without game data or a GPU.
Usage: python3 test-config.py STAGED_SYSTEM/N64/slot-native-n64.py
"""
from pathlib import Path
import importlib.util,os,sys,tempfile,configparser
p=Path(sys.argv[1]).resolve();spec=importlib.util.spec_from_file_location('slot_rice',p)
host=importlib.util.module_from_spec(spec);spec.loader.exec_module(host)
for model,panel in [('rgsp',720),('rg35xxsp',640)]:
 os.environ['SLOT_DEVICE']=model
 with tempfile.TemporaryDirectory() as tmp:
  root=Path(tmp);(root/'Config').mkdir();(root/'Games/N64').mkdir(parents=True)
  for full in [False,True]:
   (root/'Config/screen-size.ini').write_text('# independent platform settings\nPS1 = full\nN64 = '+('full' if full else 'default')+'\n')
   directory=host.make_config(root);cfg=configparser.ConfigParser(interpolation=None);cfg.read(directory/'mupen64plus.cfg')
   assert cfg.getint('Video-General','ScreenWidth')==(panel if full else 640)
   assert cfg.getint('Video-General','ScreenHeight')==480
   assert cfg.getint('Video-Rice','ResolutionWidth')==panel
   assert cfg.get('CoreEvents','Joy Mapping Save State')=='"J0B16/B2"'
   assert cfg.get('CoreEvents','Joy Mapping Load State')=='"J0B16/B3"'
  (root/'Config/n64-request').write_text('/etc/passwd')
  try:host.consume_request(root)
  except ValueError:pass
  else:raise AssertionError('outside-library request accepted')
 print('PASS',model,'native N64 panel/default/full settings, state chords and path rejection')
