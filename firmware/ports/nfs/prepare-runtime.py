from pathlib import Path
import re,sys
r=Path(sys.argv[1]).resolve()
# These acceleration hooks were located in a different executable. Keep the
# generic renderer/HLE and only the two verified original-EXE ABI replacements.
p=r/'src/runtime/overrides.c';s=p.read_text()
head=s[:s.index('/* Diagnostics')]
move=s[s.index('static void ov_crt_memmove'):s.index('static const struct',s.index('static void ov_crt_memmove'))]
p.write_text(head+move+'''nfs_override_fn nfs_override_lookup(uint32_t va) {
 if(va==0x006F5FC5u)return ov_is_cyrix;
 if(va==0x0075CFD0u)return ov_crt_memmove;
 return 0;
}
int nfs_override_count(void){return 2;}
''')
(r/'src/runtime/fx_cache.c').write_text('''#include <stdint.h>
int nfs_fx_cap_depth;
void nfs_fx_capture(uint32_t token,const uint32_t* args,uint32_t count,int deferred){}
''')
for n in ['native_math','hud_half','world_cull']:
 (r/f'src/runtime/{n}.c').write_text('/* Version-specific native hooks disabled for matching guest. */\n')
(r/'src/runtime/widescreen.c').write_text('''#include <stdint.h>
#include "nfs_widescreen.h"
void nfs_widescreen_init(const char* root){}
void nfs_widescreen_set_resolution(unsigned w,unsigned h){}
int nfs_widescreen_hud_entries(void){return 0;}
void nfs_widescreen_set_minimap_offset(float dy){}
void nfs_widescreen_set_frame_rate(unsigned fps){}
void nfs_widescreen_apply_frame_rate(void){}
void recomp_hook(uint32_t va){}
''')
p=r/'android/app/src/main/cpp/guest_runtime.cpp';s=p.read_text()
a=s.index('extern "C" void sub_005BF2E0_orig');b=s.index('#endif',a)
s=s[:a]+s[b:]
s=s.replace('if(va==0x0075bcc7u&&recomp_lookup(0x0075b9f1u))','if(va==0x0075bcc7u&&!recomp_lookup(va)&&recomp_lookup(0x0075b9f1u))')
# Do not patch resolution constants at offsets from the other EXE.
s=s.replace('if(renderWidth.load()&&renderHeight.load()){\n            const unsigned w=renderWidth.load(),h=renderHeight.load();','if(false){\n            const unsigned w=renderWidth.load(),h=renderHeight.load();')
s=s.replace("if(!(e&&e[0]=='0')){","if(e&&e[0]=='1'){")
p.write_text(s)
p=r/'android/app/src/main/cpp/arm64_guest.h';s=p.read_text();s=s.replace('if((va)==0x005BF610u){','if(0){');p.write_text(s)
print('Matching guest runtime: two verified CRT/CPU overrides; old address-specific hooks disabled.')

p=r/"ports/r36s/build_ci.sh";s=p.read_text();p.write_text(s.replace("-ldl -lm -rdynamic","-ldl -lm -lrt -rdynamic"))
