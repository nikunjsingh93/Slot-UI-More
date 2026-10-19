from pathlib import Path
import shutil,subprocess,re,concurrent.futures,time,os
import sys
root=Path(sys.argv[1]).resolve();w=Path(sys.argv[2]).resolve();w.mkdir(parents=True,exist_ok=True)
gen=root/'src/recomp/gen';out=w/'guest';out.mkdir(exist_ok=True)
assert (gen/'lift_stats.json').exists() and (gen/'recomp_dispatch.c').exists()
found=set()
split=root/'build/guest-split';split.mkdir(parents=True,exist_ok=True);sources=[]
for p in sorted(gen.glob('recomp_*.c')):
 if p.stat().st_size>8_000_000:
  subprocess.run(['python3',str(root/'scripts/split_android_sources.py'),str(p),str(split),'--max-bytes','6000000'],check=True,stdout=subprocess.DEVNULL)
  sources.extend(sorted(split.glob(p.stem+'_part_*.c')))
 else:sources.append(p)
sdk=root/'ports/r36s/sdk/include/recomp32'
flags=['-O2','-mcpu=cortex-a35','-fno-pie','-DNFS_FIXED_GUEST_BASE=0x1000000000ULL','-DRECOMP_TLS=__thread','-DRECOMP_TLS_STRUCT','-ftls-model=local-exec','-fno-math-errno','-fno-stack-clash-protection','-fno-stack-protector','-w','-fwrapv','-fno-strict-aliasing','-fexceptions','-ffp-contract=off','-g0','-fvisibility=hidden','-include',str(root/'android/app/src/main/cpp/arm64_guest.h'),'-DRECOMP_STRICT_ICALL','-DRECOMP_LOCAL_REGS','-DRECOMP_X87_WINDOW','-DRECOMP_MEM_QUAL=','-DNFS_X87_LOCAL','-I'+str(sdk),'-I'+str(gen)]
errors=w/'guest-errors';errors.mkdir(exist_ok=True)
def compile(p):
 obj=out/(p.stem+'.o');args=['gcc',*flags]
 if p.stat().st_size>6_000_000:args.append('-O1')
 started=time.monotonic()
 with (errors/(p.stem+'.log')).open('w') as f:r=subprocess.run([*args,'-c',str(p),'-o',str(obj)],stdout=f,stderr=subprocess.STDOUT)
 return p.name,r.returncode,round(time.monotonic()-started,1)
print(f'Compiling {len(sources)} matching guest units with {len(found)} native game overrides.',flush=True)
bad=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
 for i,(name,code,seconds) in enumerate(pool.map(compile,sources),1):
  print(f'{i}/{len(sources)} {name}: exit={code} elapsed={seconds}s',flush=True)
  if code:bad.append(name)
assert not bad,bad
print('All matching guest units compiled successfully.',flush=True)
