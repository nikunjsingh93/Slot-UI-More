#!/usr/bin/python3
from pathlib import Path
import sys
root=Path(sys.argv[1]);shelf=root/'Games/PORTS';shelf.mkdir(parents=True,exist_ok=True)
for title,id in [('Need for Speed Underground 2','nfsu2'),('Grand Theft Auto San Andreas','gtasa'),('PortMaster','portmaster')]:
    (shelf/(title+'.port')).write_text(id+'\n')
for script in Path('/storage/roms/ports').glob('*.sh'):
    if script.name in ['NFSU2.sh','Grand Theft Auto San Andreas.sh','PortMaster.sh']:continue
    if script.name.startswith('.') or not all(c.isascii() and (c.isalnum() or c in ' ._-()') for c in script.name):continue
    (shelf/(script.stem+'.port')).write_text('script:'+script.name+'\n')
