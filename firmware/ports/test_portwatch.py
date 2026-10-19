import os,subprocess,struct,time,sys
binary=sys.argv[1] if len(sys.argv)>1 else './slot-portwatch'
r,w=os.pipe();p=subprocess.Popen([binary,'--test-fd='+str(r),'/bin/sh','-c','sleep 30 & wait'],pass_fds=[r],stderr=subprocess.PIPE);os.close(r)
def event(value):os.write(w,struct.pack('llHHi',0,0,1,708,value))
time.sleep(.1);event(1);time.sleep(.2);event(0);time.sleep(1.05);assert p.poll() is None,'short press killed port'
event(1);start=time.monotonic();p.wait(timeout=4);assert p.returncode==0;assert time.monotonic()-start>=.95
os.close(w);print('PASS short Menu press ignored; held Menu terminates separate port process group')
r,w=os.pipe();p=subprocess.Popen([binary,'--test-fd='+str(r),'/bin/sh','-c','exit 7'],pass_fds=[r],stderr=subprocess.PIPE);os.close(r);assert p.wait(timeout=3)==7;os.close(w);print('PASS normal game exit preserves exit status')
