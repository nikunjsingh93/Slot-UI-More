"""Exercise the exact shipping folder/core/Studio provisioning shell fragment.

No handheld/system services or physical disks are accessed.
"""
from pathlib import Path
import subprocess,tempfile

repo=Path(__file__).resolve().parents[2]
source=(repo/'firmware/slot-start').read_text()
fragment=source[source.index('for platform in '):source.index('# PPSSPP requires')]
codes=['GBA','GB','GBC','PS1','SNES','N64','DC','PSP','PORTS','NES','MD','SMS','GG','PCE','ARCADE']
cores=['mgba','gpsp','pcsx_rearmed','snes9x','snes9x2005_plus','mupen64plus_next','parallel_n64','flycast','ppsspp','fceumm','genesis_plus_gx','mednafen_pce_fast','fbneo']
with tempfile.TemporaryDirectory(prefix='slot-provision-') as tmp:
    root=Path(tmp);vendor=root/'vendor';card=root/'card'
    (vendor/'Tools').mkdir(parents=True);(card/'System').mkdir(parents=True)
    for core in cores:(vendor/(core+'_libretro.so')).write_text(core)
    studio='Slot-Auto-Artwork-Studio-v5.html';(vendor/'Tools'/studio).write_text('studio-new')
    (card/'Games/NES').mkdir(parents=True);(card/'Games/NES/Keep.nes').write_bytes(b'keep existing game')
    script='set -eu\nSLOT_ROOT="'+str(card)+'"\n'+fragment.replace('/usr/share/slot',str(vendor))
    for _ in range(2):subprocess.run(['/bin/sh','-c',script],check=True)
    for group in ['Games','Labels','Saves','States']:
        for code in codes:assert (card/group/code).is_dir()
    for core in cores:assert (card/'System'/(core+'_libretro.so')).read_text()==core
    assert (card/'Tools'/studio).read_text()=='studio-new'
    assert (card/'Games/NES/Keep.nes').read_bytes()==b'keep existing game'
    custom=card/'System/fceumm_libretro.so';custom.write_text('custom core')
    (card/'Tools'/studio).write_text('older studio')
    subprocess.run(['/bin/sh','-c',script],check=True)
    assert custom.read_text()=='custom core';assert (card/'Tools'/studio).read_text()=='studio-new'
print('PASS shipping G350 fragment: 15 system folders, 13 missing cores, Studio fresh/repeat refresh, games/custom core preserved')
