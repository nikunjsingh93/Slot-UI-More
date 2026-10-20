"""Run the firmware's real tmpfiles tool in isolated first-boot directory trees.

Usage (Linux): python3 test_slot_directories.py EXTRACTED_G350_ROOT
No SD card or actual storage tree is used.
"""
from pathlib import Path
import hashlib, shutil, subprocess, sys, tempfile

def main():
    root=Path(sys.argv[1]).resolve()
    config=Path(__file__).with_name('slot-directories.conf').read_bytes()
    legacy=['3do','advision','amiga','amiga1200','amigacd32','amstradcpc','arcade','arduboy','atari2600','atari5200','atari7800','atari800','atarijaguar','atarilynx','atarist','atomiswave','c128','c16','c64','cdi','channelf','chip-8','coleco','cps1','cps2','cps3','doom','dreamcast','easyrpg','ecwolf','eduke','ep64-128','famicom','fbneo','fds','gamate','gameandwatch','gamegear','gameking','gamemaster','gamepocketcomputer','gb','gba','gbc','genesis','gx4000','intellivision','j2me','laserdisc','lowresnx','mame','mastersystem','megacd','megadrive','megadrive-japan','megaduck','microw8','mplayer','msx','msx2','n64','naomi','nds','neocd','neogeo','nes','ngp','ngpc','odyssey','openbor','pc','pc88','pc98','pcengine','pcenginecd','pcfx','pico-8','piece','pokemini','psp','pspminis','psx','pv1000','satellaview','saturn','sc-3000','scummvm','scv','sega32x','segacd','sfc','sg-1000','sgfx','snes','snesmsu1','solarus','sufami','supervision','tg16','tg16cd','thomson','tic-80','tvc','uzebox','vectrex','vic20','videopac','vircon32','virtualboy','wasm4','wonderswan','wonderswancolor','x1','x68000','zmachine','zx81','zxspectrum']
    assert len(legacy)==117
    (root/'tmp').mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='slot-directory-check-',dir=root/'tmp') as name:
        case=Path(name)
        rule=case/'usr/lib/tmpfiles.d/amberelec-dirs.conf';rule.parent.mkdir(parents=True);rule.write_bytes(config)
        (case/'etc').mkdir();(case/'etc/passwd').write_text('root:x:0:0:root:/root:/bin/sh\n');(case/'etc/group').write_text('root:x:0:\n')
        # Use the firmware's executable AND loader/libraries, while allowing
        # access to the container's /proc (required by systemd-tmpfiles).
        libraries=':'.join(str(root/p) for p in ['usr/lib','usr/lib/systemd'])
        cmd=[str(root/'usr/lib/ld-linux-aarch64.so.1'),'--library-path',libraries,str(root/'usr/bin/systemd-tmpfiles'),'--create','--boot','--root='+str(case)]
        def run():
            result=subprocess.run(cmd,capture_output=True,text=True)
            assert result.returncode==0,result.stdout+result.stderr
        for _ in range(2):run()
        roms=case/'storage/roms'
        assert not any((roms/p).exists() for p in legacy)
        assert not (roms/'homebrew').exists()
        for p in ['Slot','ports','bios','gamedata','savestates','screenshots','update','bezels','BGM','build']:assert (roms/p).is_dir(),p
        # Existing libraries, native port data and even populated legacy folders survive.
        sentinels={}
        for p in ['Slot/Games/GBA/Local.gba','Slot/Saves/PS1/Local.srm','Slot/Config/slot.state','ports/local/data.bin','snes/Old Game.sfc','homebrew/Keep.txt']:
            target=roms/p;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(('preserve '+p).encode());sentinels[target]=hashlib.sha256(target.read_bytes()).hexdigest()
        run()
        for p,h in sentinels.items():assert hashlib.sha256(p.read_bytes()).hexdigest()==h,p
    print('PASS real G350 tmpfiles: first/repeated boot creates no 117 legacy system folders; Slot/native paths exist; existing games, saves, config, native ports and old populated folders survive.')

if __name__=='__main__':main()
