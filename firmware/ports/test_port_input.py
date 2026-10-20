"""Exercise the real supervisor and controller forwarding with synthetic Linux input.
No ROMs, physical card, game binaries, mixer or privileged uinput are used.
Kernel virtual-device creation and handheld behavior still need device testing.
"""
from pathlib import Path
import os,struct,subprocess,sys,tempfile,time
binary=str(Path(sys.argv[1]).resolve())
event=struct.Struct('llHHi')
with tempfile.TemporaryDirectory() as tmp:
 root=Path(tmp);(root/'Config').mkdir()
 (root/'Config/slot.state').write_text('volume=35\nshader=4\nfuture=keep\n')
 def start(name,device='rgsp'):
  r,w=os.pipe();sink=root/'events';sink.write_bytes(b'')
  env=dict(os.environ,SLOT_DEVICE=device,SLOT_MENU_CODE='312',SLOT_ROOT=tmp,
           SLOT_PORT_ID='script:'+name,SLOT_INPUT_TEST_OUTPUT=str(sink))
  p=subprocess.Popen([binary,'--test-fd='+str(r),'--test-volume='+tmp,'/bin/sh','-c','sleep 30 & wait'],pass_fds=[r],env=env,stderr=subprocess.PIPE)
  os.close(r);time.sleep(.15);assert p.poll() is None,p.stderr.read().decode()
  def send(t,c,v):os.write(w,event.pack(0,0,t,c,v))
  def events():
   b=sink.read_bytes();return [(t,c,v) for _,_,t,c,v in event.iter_unpack(b)]
  return p,w,send,events
 def wait(check):
  for _ in range(80):
   if check():return
   time.sleep(.025)
  raise AssertionError('Expected input state not observed')
 def close(p,w):
  p.terminate();p.wait(timeout=5);os.close(w)
 for device in ['rgsp','rg35xxsp']:
  name=device+'.sh';setting=root/'Config/port-input'/ (name+'.mode')
  p,w,send,events=start(name,device)
  try:
   send(3,16,-1);send(3,17,1);send(1,304,1);send(0,0,0)
   wait(lambda:(3,16,-1) in events())
   send(1,314,1);send(1,314,0)  # L2 is not Select on these SP pads.
   wait(lambda:(1,314,1) in events() and (1,314,0) in events())
   send(1,312,1);send(1,310,1);send(1,310,2)
   wait(lambda:setting.exists() and setting.read_text()=='1\n')
   time.sleep(1.2);assert p.poll() is None,'Chord was mistaken for Menu-alone exit'
   e=events();assert (3,16,0) in e and (3,17,0) in e
   assert (3,0,-32767) in e and (3,1,32767) in e
   assert (1,304,1) in e and (1,310,1) not in e and (1,312,1) not in e
   send(1,310,0);send(1,312,0);send(3,16,0);send(3,17,0);send(0,0,0)
   wait(lambda:(3,0,0) in events() and (3,1,0) in events())
   send(1,310,1);send(1,310,0);send(0,0,0)
   wait(lambda:(1,310,1) in events() and (1,310,0) in events())
   send(1,312,1);send(1,310,1)
   wait(lambda:setting.read_text()=='0\n')
   send(1,310,0);send(1,312,0)
   send(1,312,1);send(1,310,1)
   wait(lambda:setting.read_text()=='1\n')
   send(1,310,0);send(1,312,0)
   send(1,312,1);then=time.monotonic();p.wait(timeout=5)
   assert p.returncode==0 and time.monotonic()-then>=.95
  finally:
   if p.poll() is None:p.terminate();p.wait(timeout=5)
   os.close(w)
  p,w,send,events=start(name,device)
  try:
   send(3,16,1);send(0,0,0);wait(lambda:(3,0,32767) in events())
   assert (3,16,1) not in events(),'Remembered mode was not restored'
  finally:close(p,w)
  p,w,send,events=start(device+'-another.sh',device)
  try:
   send(3,16,1);send(0,0,0);wait(lambda:(3,16,1) in events())
   assert (3,0,32767) not in events(),'Mode leaked to another game'
  finally:close(p,w)
 assert 'future=keep' in (root/'Config/slot.state').read_text()
 print('PASS both SPs: digital/analog diagonals, neutral release, chord suppression/repeat, per-game persistence/isolation, Menu-alone exit and volume-state preservation')

 # Game-specific GTA layout keeps the known working G350 semantics by position.
 # Unlike the generic SP pad, its virtual pad is canonical GTASA-Gamepad.
 for device in ['rgsp','rg35xxsp']:
  # Remove the first device's test mode so both exercise the analog default.
  setting=root/'Config/port-input/Grand Theft Auto San Andreas.sh.mode'
  setting.unlink(missing_ok=True)
  p,w,send,events=start('Grand Theft Auto San Andreas.sh',device)
  try:
   send(3,16,-1);send(3,17,1);send(0,0,0)
   wait(lambda:(3,0,-32767) in events() and (3,1,32767) in events())
   for physical,canonical in [(304,305),(305,304),(306,308),(307,307),(308,310),(309,311),(310,704),(311,705),(314,312),(315,313)]:
    offset=len(events());send(1,physical,1);send(1,physical,0);send(0,0,0)
    wait(lambda:(1,canonical,1) in events()[offset:] and (1,canonical,0) in events()[offset:])
   send(1,354,1);send(1,354,0);time.sleep(.03)
   assert not any(code==354 for t,code,v in events() if t==1)
   send(1,312,1);send(1,310,1);wait(lambda:setting.exists() and setting.read_text()=='0\n')
   send(1,310,0);send(1,312,0)
  finally:close(p,w)
  p,w,send,events=start('Grand Theft Auto San Andreas.sh',device)
  try:
   send(3,16,-1);send(0,0,0);wait(lambda:(3,16,-1) in events())
   assert (3,0,-32767) not in events(),'Saved digital mode was overridden by GTA default'
  finally:close(p,w)
 print('PASS GTA on both SPs: analog default, physical face/shoulder/Select/Start table, Menu echo removal and persisted digital override')

 p,w,send,events=start('Need for Speed Underground 2.sh')
 try:
  send(3,16,1);send(0,0,0);wait(lambda:(3,0,32767) in events())
  send(1,312,1);send(1,307,1);send(1,307,2)
  wait(lambda:(1,707,1) in events());time.sleep(1.1)
  assert p.poll() is None,'NFS keyboard chord was mistaken for Menu-alone exit'
  send(1,307,0);send(1,312,0);wait(lambda:(1,707,0) in events())
  assert (1,307,1) not in events(),'Keyboard chord leaked the normal game button'
  send(1,312,1);send(1,310,1)
  setting=root/'Config/port-input/Need for Speed Underground 2.sh.mode'
  wait(lambda:setting.exists() and setting.read_text()=='0\n')
  send(1,310,0);send(1,312,0);send(3,16,-1);send(0,0,0)
  wait(lambda:(1,546,1) in events())
 finally:close(p,w)
 print('PASS NFS: analog default, digital D-pad buttons, Menu+X keyboard toggle and preserved Menu quit handling')
