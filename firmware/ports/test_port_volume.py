"""Exercise the real supervisor with synthetic evdev input and a mixer sink.
No game files or physical audio/input devices are needed.
"""
from pathlib import Path
import os,subprocess,tempfile,time,struct,sys,math
binary=str(Path(sys.argv[1]).resolve())
with tempfile.TemporaryDirectory() as tmp:
    root=Path(tmp);(root/'Config').mkdir()
    state=root/'Config/slot.state'
    state.write_text('cart=keep.port\nshader=4\nvolume=10\nvolume_hp=60\nmuted=0\nmuted_hp=0\nfuture_key=keep me\n')
    (root/'gpio').write_text('0')
    r,w=os.pipe()
    child='printf "%s" "$AUDIODEV" > "'+tmp+'/device"; cp "$ALSA_CONFIG_PATH" "'+tmp+'/alsa-copy"; sleep 30 & wait'
    p=subprocess.Popen([binary,'--test-fd='+str(r),'--test-volume='+tmp,'/bin/sh','-c',child],pass_fds=[r],stderr=subprocess.PIPE)
    os.close(r)
    def event(code,value):os.write(w,struct.pack('llHHi',0,0,1,code,value))
    def values():return dict(line.split('=',1) for line in state.read_text().splitlines() if '=' in line)
    def mixer():return tuple(map(int,(root/'mixer.log').read_text().splitlines()[-1].split()))
    def until(check,seconds=2):
        deadline=time.monotonic()+seconds
        while time.monotonic()<deadline:
            assert p.poll() is None,'Supervisor exited: '+p.stderr.read().decode()
            try:
                if check():return
            except (FileNotFoundError,IndexError):pass
            time.sleep(.025)
        raise AssertionError('Timed out waiting for expected state')
    def tap(code):event(code,1);event(code,0)
    try:
        until(lambda:(root/'device').exists() and (root/'alsa-copy').exists())
        assert (root/'device').read_text()=='slot_ports'
        cfg=(root/'alsa-copy').read_text();assert 'type softvol' in cfg and 'resolution 1001' in cfg
        assert mixer()==(0,10,0,500),'Startup did not inherit 10% speaker volume'
        tap(115);until(lambda:values()['volume']=='15')
        time.sleep(.45);assert values()['volume']=='15','Released key kept repeating'
        event(114,1);time.sleep(.75);event(114,0)
        until(lambda:values()['volume']=='0');assert mixer()[-1]==0
        tap(115);until(lambda:values()['volume']=='5')
        (root/'gpio').write_text('1');until(lambda:mixer()[:3]==(1,60,0))
        event(115,1);until(lambda:int(values()['volume_hp'])>=70);event(115,0);time.sleep(.05)
        hp=values()['volume_hp'];assert values()['volume']=='5'
        event(115,1);time.sleep(.06);event(114,1)
        until(lambda:values()['muted_hp']=='1')
        time.sleep(.5);assert values()['volume_hp']==hp and mixer()[-1]==0
        event(115,0);event(114,0)
        event(115,1);time.sleep(.06);event(114,1)
        until(lambda:values()['muted_hp']=='0' and values()['volume_hp']==hp)
        event(115,0);event(114,0)
        assert values()['volume_hp']==hp,'Mute chord changed the remembered level'
        (root/'gpio').write_text('0');until(lambda:mixer()[:3]==(0,5,0))
        assert values()['future_key']=='keep me' and values()['shader']=='4' and values()['cart']=='keep.port'
        # Every applied setting matches Slot's squared gain, allowing softvol's
        # finite control/Q16 precision. Zero/mute must be true silence.
        for line in (root/'mixer.log').read_text().splitlines():
            _,level,mute,raw=map(int,line.split())
            if level==0 or mute:assert raw==0;continue
            gain=round(10**((-80+raw*80/1000)/20)*65536)/65536
            assert abs(gain-(level/100)**2)/(level/100)**2<.015
        event(708,1);time.sleep(.15);event(708,0);time.sleep(.15);assert p.poll() is None
        event(708,1);assert p.wait(timeout=4)==0
        print('PASS saved startup level, rocker taps/hold/release, zero, mute chord, speaker/headphone profiles, state preservation and Menu-hold quit')
    finally:
        if p.poll() is None:p.terminate();p.wait(timeout=5)
        os.close(w)
    # Older Slot state files have one shared profile. Match the frontend's
    # fallback rather than unexpectedly unmuting plugged-in headphones.
    state.write_text('volume=20\nmuted=1\nfuture_key=keep me\n')
    (root/'gpio').write_text('1')
    r,w=os.pipe()
    p=subprocess.Popen([binary,'--test-fd='+str(r),'--test-volume='+tmp,'/bin/sh','-c','exit 7'],pass_fds=[r],stderr=subprocess.PIPE)
    os.close(r)
    try:
        assert p.wait(timeout=3)==7
        assert mixer()==(1,20,1,0),'Legacy headphone profile did not inherit volume/mute'
        print('PASS legacy volume/mute inheritance and normal port exit')
    finally:os.close(w)
