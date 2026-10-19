/* Native ports use ALSA softvol; frontend/emulator gain remains unchanged. */
#include <alsa/asoundlib.h>
#include <math.h>
#include <ctype.h>
#include <limits.h>
typedef struct {
 char state[PATH_MAX],gpio[PATH_MAX],config[PATH_MAX],mock[PATH_MAX];
 int level[2],mute[2],before[2],before_mute[2],hp,chord,enabled;
 long down[2],repeat[2],hp_at;
 snd_ctl_t *ctl;
} port_volume;
static int bounded(int n){return n<0?0:n>100?100:n;}
/* Slot's sample gain is (percent / 100)^2; softvol has 1001 steps -80..0 dB. */
static int volume_raw(int n,int muted){
 if(muted||n<=0)return 0;
 double db=40.0*log10(bounded(n)/100.0);
 int raw=(int)lround((db+80.0)*1000.0/80.0);
 return raw<1?1:raw>1000?1000:raw;
}
static int read_hp(const char *path){FILE*f=fopen(path,"r");int n=0;if(f){if(!strcmp(path,"/sys/kernel/debug/gpio")){char line[512];while(fgets(line,sizeof(line),f))if(strstr(line,"|Headphone detection")){char*q=strrchr(line,')');if(q&&strstr(q," hi"))n=1;}}else fscanf(f,"%d",&n);fclose(f);}return n==1;}
static int volume_apply(port_volume*v){
 int raw=volume_raw(v->level[v->hp],v->mute[v->hp]);
 if(v->mock[0]){
  FILE*f=fopen(v->mock,"a");if(!f)return -1;
  fprintf(f,"%d %d %d %d\n",v->hp,v->level[v->hp],v->mute[v->hp],raw);fclose(f);
 }else{
  snd_ctl_elem_value_t *value;snd_ctl_elem_value_alloca(&value);
  snd_ctl_elem_value_set_interface(value,SND_CTL_ELEM_IFACE_MIXER);
  snd_ctl_elem_value_set_name(value,"Slot Ports Playback Volume");
  snd_ctl_elem_value_set_integer(value,0,raw);
  int err=snd_ctl_elem_write(v->ctl,value);if(err<0){fprintf(stderr,"slot ports: volume write: %s\n",snd_strerror(err));return -1;}
 }
 fprintf(stderr,"slot ports: %s volume=%d muted=%d\n",v->hp?"headphones":"speaker",v->level[v->hp],v->mute[v->hp]);
 return 0;
}
static int volume_save(port_volume*v){
 char tmp[PATH_MAX];if(snprintf(tmp,sizeof(tmp),"%s.port-tmp",v->state)>=(int)sizeof(tmp))return -1;
 FILE*in=fopen(v->state,"r"),*out=fopen(tmp,"w");if(!out){if(in)fclose(in);return -1;}
 char*line=NULL;size_t cap=0;
 while(in&&getline(&line,&cap,in)>=0){
  if(!strncmp(line,"volume=",7)||!strncmp(line,"volume_hp=",10)||!strncmp(line,"muted=",6)||!strncmp(line,"muted_hp=",9))continue;
  fputs(line,out);
  if(line[0]&&line[strlen(line)-1]!='\n')fputc('\n',out);
 }
 free(line);if(in)fclose(in);
 fprintf(out,"volume=%d\nvolume_hp=%d\nmuted=%d\nmuted_hp=%d\n",v->level[0],v->level[1],v->mute[0],v->mute[1]);
 int failed=fflush(out);if(!failed)failed=fsync(fileno(out));if(fclose(out))failed=1;
 if(failed||rename(tmp,v->state)){unlink(tmp);return -1;}
 char dir[PATH_MAX];snprintf(dir,sizeof(dir),"%s",v->state);char*slash=strrchr(dir,'/');if(slash){*slash=0;int fd=open(dir,O_RDONLY|O_DIRECTORY);if(fd>=0){fsync(fd);close(fd);}}
 return 0;
}
static int volume_changed(port_volume*v){
 if(volume_apply(v)<0)return -1;
 if(volume_save(v)<0){fprintf(stderr,"slot ports: could not persist volume: %s\n",strerror(errno));return -1;}
 return 0;
}
static int volume_init(port_volume*v,const char*root,const char*test){
 memset(v,0,sizeof(*v));v->level[0]=v->level[1]=60;
 if(snprintf(v->state,sizeof(v->state),"%s/Config/slot.state",root)>=(int)sizeof(v->state))return -1;
 snprintf(v->gpio,sizeof(v->gpio),"%s", getenv("SLOT_HP_PATH")?getenv("SLOT_HP_PATH"):"/sys/class/gpio/gpio86/value");
 if(test){snprintf(v->gpio,sizeof(v->gpio),"%s/gpio",test);snprintf(v->mock,sizeof(v->mock),"%s/mixer.log",test);}
 FILE*f=fopen(v->state,"r");char*line=NULL;size_t cap=0;int value,has_hp=0,has_mute_hp=0;
 while(f&&getline(&line,&cap,f)>=0){
  if(sscanf(line,"volume=%d",&value)==1)v->level[0]=bounded(value);
  else if(sscanf(line,"volume_hp=%d",&value)==1){v->level[1]=bounded(value);has_hp=1;}
  else if(sscanf(line,"muted=%d",&value)==1)v->mute[0]=value!=0;
  else if(sscanf(line,"muted_hp=%d",&value)==1){v->mute[1]=value!=0;has_mute_hp=1;}
 }
 free(line);if(f)fclose(f);if(!has_hp)v->level[1]=v->level[0];if(!has_mute_hp)v->mute[1]=v->mute[0];v->hp=read_hp(v->gpio);
 snprintf(v->config,sizeof(v->config),"/tmp/slot-port-audio-XXXXXX");int fd=mkstemp(v->config);if(fd<0)return -1;
 FILE*cfg=fdopen(fd,"w");if(!cfg){close(fd);unlink(v->config);return -1;}
 const char*device=getenv("SLOT_ALSA_DEVICE");if(!device||!*device)device="plughw:0,0";
 for(const char*p=device;*p;p++)if(!isalnum((unsigned char)*p)&&!strchr("_:,=.-",*p)){fclose(cfg);unlink(v->config);return -1;}
 const char*card_env=getenv("SLOT_ALSA_CARD");int card=card_env?atoi(card_env):0;if(card<0||card>31){fclose(cfg);unlink(v->config);return -1;}
 fprintf(cfg,"</usr/share/alsa/alsa.conf>\npcm.slot_ports_gain { type softvol slave { pcm \"%s\" format S16_LE } control { name \"Slot Ports Playback Volume\" card %d count 1 } min_dB -80.0 max_dB 0.0 resolution 1001 }\npcm.slot_ports { type plug slave.pcm \"slot_ports_gain\" }\npcm.!default { type plug slave.pcm \"slot_ports\" }\n",device,card);
 if(fclose(cfg)){unlink(v->config);return -1;}
 setenv("ALSA_CONFIG_PATH",v->config,1);setenv("SLOT_PORT_PCM","slot_ports",1);setenv("AUDIODEV","slot_ports",1);
 if(!test){
  snd_pcm_t*pcm=NULL;int err=snd_pcm_open(&pcm,"slot_ports",SND_PCM_STREAM_PLAYBACK,0);
  if(err<0){fprintf(stderr,"slot ports: softvol open: %s\n",snd_strerror(err));unlink(v->config);return -1;}
  snd_pcm_close(pcm);
  char control[32];snprintf(control,sizeof(control),"hw:%d",card);err=snd_ctl_open(&v->ctl,control,0);
  if(err<0){unlink(v->config);return -1;}
  snd_ctl_elem_info_t*info;snd_ctl_elem_info_alloca(&info);
  snd_ctl_elem_info_set_interface(info,SND_CTL_ELEM_IFACE_MIXER);snd_ctl_elem_info_set_name(info,"Slot Ports Playback Volume");
  err=snd_ctl_elem_info(v->ctl,info);
  if(err<0||snd_ctl_elem_info_get_min(info)!=0||snd_ctl_elem_info_get_max(info)!=1000){snd_ctl_close(v->ctl);v->ctl=NULL;unlink(v->config);return -1;}
 }
 v->enabled=1;return volume_apply(v);
}
static void volume_reset_keys(port_volume*v){v->down[0]=v->down[1]=v->repeat[0]=v->repeat[1]=0;v->chord=0;}
static void volume_step(port_volume*v,int key){v->level[v->hp]=bounded(v->level[v->hp]+(key==0?5:-5));v->mute[v->hp]=0;volume_changed(v);}
static void volume_event(port_volume*v,const struct input_event*e,long now){
 if(!v->enabled||e->type!=EV_KEY||(e->code!=KEY_VOLUMEUP&&e->code!=KEY_VOLUMEDOWN))return;
 int key=e->code==KEY_VOLUMEUP?0:1,other=1-key;
 if(e->value==0){v->down[key]=v->repeat[key]=0;if(!v->down[other])v->chord=0;return;}
 if(e->value!=1||v->down[key])return;
 if(!v->down[other]){memcpy(v->before,v->level,sizeof(v->level));memcpy(v->before_mute,v->mute,sizeof(v->mute));}
 v->down[key]=now;v->repeat[key]=now+400;
 if(v->down[other]&&now-v->down[other]<=200&&!v->chord){
  memcpy(v->level,v->before,sizeof(v->level));memcpy(v->mute,v->before_mute,sizeof(v->mute));v->mute[v->hp]=!v->mute[v->hp];v->chord=1;volume_changed(v);
 }else if(!v->chord)volume_step(v,key);
}
static void volume_tick(port_volume*v,long now){
 if(!v->enabled)return;
 if(now-v->hp_at>=200){v->hp_at=now;int hp=read_hp(v->gpio);if(hp!=v->hp){v->hp=hp;volume_reset_keys(v);volume_apply(v);}}
 if(!v->chord&&!!v->down[0]!=!!v->down[1]){
  int key=v->down[0]?0:1;if(now>=v->repeat[key]){volume_step(v,key);v->repeat[key]=now+120;}
 }
}
static void volume_close(port_volume*v){if(v->ctl)snd_ctl_close(v->ctl);if(v->config[0])unlink(v->config);}
