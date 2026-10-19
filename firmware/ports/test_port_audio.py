"""Measure real ALSA softvol PCM output; only the absent hardware control is mocked.
Optional first argument selects an extracted G350 firmware root/library loader.
"""
from pathlib import Path
import tempfile,subprocess,os,struct,sys
source=Path(__file__).resolve().parent
with tempfile.TemporaryDirectory() as tmp:
 p=Path(tmp);(p/'Config').mkdir();(p/'Config/slot.state').write_text('volume=10\nvolume_hp=60\nmuted=0\nmuted_hp=0\n')
 (p/'.asoundrc').write_text('pcm.slot_test_sink { type file slave.pcm "null" file "'+str(p/'audio.raw')+'" format "raw" }\n')
 (p/'mock.c').write_text('''#include <sound/asound.h>
#include <stdint.h>
#include <errno.h>
static long volume=1000;
int snd_ctl_open(void**ctl,const char*name,int mode){*ctl=(void*)(uintptr_t)1;return 0;}
int snd_ctl_close(void*ctl){return 0;}
int snd_ctl_elem_info(void*ctl,struct snd_ctl_elem_info*info){
 info->type=SNDRV_CTL_ELEM_TYPE_INTEGER;info->access=SNDRV_CTL_ELEM_ACCESS_USER|SNDRV_CTL_ELEM_ACCESS_READWRITE|SNDRV_CTL_ELEM_ACCESS_TLV_READ|SNDRV_CTL_ELEM_ACCESS_TLV_WRITE;
 info->count=1;info->value.integer.min=0;info->value.integer.max=1000;return 0;}
int snd_ctl_elem_read(void*ctl,struct snd_ctl_elem_value*value){value->value.integer.value[0]=volume;return 0;}
int snd_ctl_elem_write(void*ctl,struct snd_ctl_elem_value*value){volume=value->value.integer.value[0];return 0;}
int snd_ctl_elem_tlv_read(void*ctl,const struct snd_ctl_elem_id*id,unsigned int*tlv,unsigned int size){return -ENOENT;}
int snd_ctl_elem_tlv_write(void*ctl,const struct snd_ctl_elem_id*id,const unsigned int*tlv){return 0;}
''')
 (p/'audio.c').write_text('''#define main unused_portwatch_main
#include "portwatch.c"
#undef main
int main(int argc,char**argv){
 port_volume v;if(volume_init(&v,argv[1],NULL)<0)return 10;
 snd_pcm_t*pcm;if(snd_pcm_open(&pcm,"slot_ports",SND_PCM_STREAM_PLAYBACK,0)<0)return 11;
 int err=snd_pcm_set_params(pcm,SND_PCM_FORMAT_S16_LE,SND_PCM_ACCESS_RW_INTERLEAVED,2,48000,1,100000);
 if(err<0){fprintf(stderr,"params: %s\\n",snd_strerror(err));return 12;}
 short data[128];for(int i=0;i<128;i++)data[i]=i%2?-10000:10000;
 int levels[]={10,50,100,0};
 for(int n=0;n<4;n++){v.level[0]=levels[n];if(volume_apply(&v)<0)return 13;if(snd_pcm_writei(pcm,data,64)!=64)return 14;}
 snd_pcm_drain(pcm);snd_pcm_close(pcm);volume_close(&v);return 0;
}''')
 subprocess.run(['gcc','-shared','-fPIC',str(p/'mock.c'),'-o',str(p/'mock.so')],check=True)
 subprocess.run(['gcc','-O2','-I'+str(source),str(p/'audio.c'),'-lasound','-lm','-o',str(p/'audio')],check=True)
 env=dict(os.environ,HOME=str(p),SLOT_ALSA_DEVICE='slot_test_sink',LD_PRELOAD=str(p/'mock.so'))
 command=[str(p/'audio'),str(p)]
 if len(sys.argv)>1:
  fw=Path(sys.argv[1]);command=[str(fw/'usr/lib/ld-linux-aarch64.so.1'),'--library-path',str(fw/'usr/lib'),*command]
 subprocess.run(command,env=env,check=True)
 data=struct.unpack('<512h',(p/'audio.raw').read_bytes())
 for n,expected in enumerate([100,2500,10000,0]):
  block=data[n*128:(n+1)*128]
  assert len(block)==128 and all(abs(abs(x)-expected)<=max(2,expected*.006) for x in block),(n,min(block),max(block))
 print('PASS actual ALSA PCM softvol samples: 10%=100, 50%~2500, 100%=10000, 0%=silence (input peak 10000). Hardware control only is mocked.')
