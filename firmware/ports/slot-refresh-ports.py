#!/usr/bin/python3
from pathlib import Path
import sys,os
root=Path(sys.argv[1]);shelf=root/'Games/PORTS';shelf.mkdir(parents=True,exist_ok=True)
entries=[('PortMaster','portmaster')]
if os.environ.get('SLOT_DEVICE','g350')=='g350':entries += [('Need for Speed Underground 2','nfsu2'),('Grand Theft Auto San Andreas','gtasa')]
for title,id in entries:
    (shelf/(title+'.port')).write_text(id+'\n')
for script in Path(os.environ.get('SLOT_PORTS_DIR','/storage/roms/ports')).glob('*.sh'):
    if script.name=='PortMaster.sh' or (os.environ.get('SLOT_DEVICE','g350')=='g350' and script.name in ['NFSU2.sh','Grand Theft Auto San Andreas.sh']):continue
    if script.name.startswith('.') or not all(c.isascii() and (c.isalnum() or c in ' ._-()') for c in script.name):continue
    (shelf/(script.stem+'.port')).write_text('script:'+script.name+'\n')
