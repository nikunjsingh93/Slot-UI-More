/* SP native ports: one virtual copy of the built-in pad, owned by portwatch.
 * Other controllers and the G350 path are untouched. */
#include <linux/uinput.h>
#include <sched.h>
#include <sys/mount.h>
#include <sys/stat.h>
#include <dirent.h>
#include <limits.h>
typedef struct {
 int active, fd, source, mode, menu, select, chord, suppress_select;
 int x,y, test, menu_code, gta, nfs, n64, canonical, r3_down;
 char setting[PATH_MAX], event[PATH_MAX], js[PATH_MAX];
} port_input;
static int input_save(port_input *p) {
 char tmp[PATH_MAX+32];snprintf(tmp,sizeof(tmp),"%s.tmp.%ld",p->setting,(long)getpid());
 int fd=open(tmp,O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC|O_NOFOLLOW,0600);
 if(fd<0)return -1;
 char value[2]={(char)('0'+p->mode),'\n'};
 int ok=write(fd,value,2)==2&&fsync(fd)==0;
 if(close(fd)<0)ok=0;
 if(ok&&rename(tmp,p->setting)==0)return 0;
 unlink(tmp);return -1;
}
static void input_emit(port_input *p,unsigned short type,unsigned short code,int value) {
 struct input_event e={0};e.type=type;e.code=code;e.value=value;
 if(write(p->fd,&e,sizeof(e))!=(ssize_t)sizeof(e)) {
  fprintf(stderr,"slot ports: virtual input write failed: %s\n",strerror(errno));
  stopped=1;
 }
}
static void input_directions(port_input *p) {
 if(p->canonical){
  input_emit(p,EV_KEY,544,!p->mode&&p->y<0);input_emit(p,EV_KEY,545,!p->mode&&p->y>0);
  input_emit(p,EV_KEY,546,!p->mode&&p->x<0);input_emit(p,EV_KEY,547,!p->mode&&p->x>0);
 }
 input_emit(p,EV_ABS,ABS_HAT0X,p->mode?0:p->x);
 input_emit(p,EV_ABS,ABS_HAT0Y,p->mode?0:p->y);
 input_emit(p,EV_ABS,ABS_X,p->mode?p->x*32767:0);
 input_emit(p,EV_ABS,ABS_Y,p->mode?p->y*32767:0);
 input_emit(p,EV_SYN,SYN_REPORT,0);
}
static int input_paths(port_input *p,const char *root,const char *id) {
 if(!id||strncmp(id,"script:",7))return 0; /* manager is always digital */
 id+=7;size_t len=strlen(id);
 if(!len||len>120||id[0]=='.'||len<3||strcmp(id+len-3,".sh"))return -1;
 for(const unsigned char *s=(const unsigned char*)id;*s;s++)
  if(!((*s>='a'&&*s<='z')||(*s>='A'&&*s<='Z')||(*s>='0'&&*s<='9')||strchr(" ._()-",*s)))return -1;
 char dir[PATH_MAX];if(snprintf(dir,sizeof(dir),"%s/Config/port-input",root)>=(int)sizeof(dir))return -1;
 if(mkdir(dir,0755)<0&&errno!=EEXIST)return -1;
 if(snprintf(p->setting,sizeof(p->setting),"%s/%s.mode",dir,id)>=(int)sizeof(p->setting))return -1;
 int fd=open(p->setting,O_RDONLY|O_CLOEXEC|O_NOFOLLOW);char c;
 if(fd>=0){if(read(fd,&c,1)==1&&(c=='0'||c=='1'))p->mode=c-'0';close(fd);}
 return 1;
}
/* Preserve the working R36S GTA button semantics by physical position.
 * H700 raw layout: A304 B305 Y306 X307 L1308 R1309 Select310 Start311,
 * Menu312 L2314 R2315. This profile is only used by the named GTA launcher. */
static int input_key(port_input *p,int key) {
 if(!p->canonical)return key;
 switch(key){case 304:return 305;case 305:return 304;case 306:return 308;
 case 307:return 307;case 308:return 310;case 309:return 311;
 case 310:return 704;case 311:return 705;case 314:return 312;case 315:return 313;
 default:return key;}
}
static int input_pad(int fd) {
 const char *device=getenv("SLOT_DEVICE");
 if(!device||(strcmp(device,"rgsp")&&strcmp(device,"rg35xxsp")))return 0;
 unsigned long keys[(KEY_MAX+8*sizeof(long))/(8*sizeof(long))]={0},axes[(ABS_MAX+8*sizeof(long))/(8*sizeof(long))]={0};
 struct input_id id={0};
 return ioctl(fd,EVIOCGID,&id)>=0&&id.bustype==BUS_HOST&&ioctl(fd,EVIOCGBIT(EV_KEY,sizeof(keys)),keys)>=0&&bit(keys,BTN_SOUTH)&&
  ioctl(fd,EVIOCGBIT(EV_ABS,sizeof(axes)),axes)>=0&&bit(axes,ABS_HAT0X)&&bit(axes,ABS_HAT0Y);
}
static int input_init(port_input *p,const char *root,const char *id,struct pollfd *fds,int n,const char *test) {
 p->fd=-1;p->source=-1;p->menu_code=312;
 const char *device=getenv("SLOT_DEVICE");
 if(!device||(strcmp(device,"rgsp")&&strcmp(device,"rg35xxsp")))return 0;
 if(!root)return -1;
 p->gta=id&&!strcmp(id,"script:Grand Theft Auto San Andreas.sh");
 p->nfs=id&&!strcmp(id,"script:Need for Speed Underground 2.sh");
 p->n64=id&&!strcmp(id,"script:Slot Nintendo 64.sh");
 /* SDL's H700 fixed-layout driver ignores newly added ABS_X/Y on an
  * ANBERNIC-named pad. Use the existing normalized pad for these SDL games. */
 int sdl_port=id&&(!strcmp(id,"script:Grand Theft Auto 3.sh")||
     !strcmp(id,"script:Grand Theft Auto Vice City.sh")||
     !strcmp(id,"script:RVGL.sh")||!strcmp(id,"script:SuperTuxKart.sh")||
     !strcmp(id,"script:Need for Speed Most Wanted.sh"));
 p->canonical=p->gta||p->nfs||p->n64||sdl_port;p->mode=p->canonical;
 int enabled=input_paths(p,root,id);if(enabled<=0)return enabled;
 if(test) {
  p->test=1;p->source=0;p->fd=open(test,O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC,0600);
  if(p->fd<0)return -1;
  p->active=1;return 1;
 }
 for(int i=0;i<n;i++)if(input_pad(fds[i].fd)){p->source=i;break;}
 if(p->source<0){fprintf(stderr,"slot ports: built-in hat gamepad not found\n");return -1;}
 int source=fds[p->source].fd;
 /* uinput is opened before grabbing anything; failed setup never steals input. */
 p->fd=open("/dev/uinput",O_WRONLY|O_NONBLOCK|O_CLOEXEC);
 if(p->fd<0)p->fd=open("/dev/input/uinput",O_WRONLY|O_NONBLOCK|O_CLOEXEC);
 if(p->fd<0) {
  pid_t load=fork();
  if(load==0){execlp("modprobe","modprobe","uinput",(char*)NULL);_exit(127);}
  if(load>0){int status;while(waitpid(load,&status,0)<0&&errno==EINTR){}}
  p->fd=open("/dev/uinput",O_WRONLY|O_NONBLOCK|O_CLOEXEC);
  if(p->fd<0)p->fd=open("/dev/input/uinput",O_WRONLY|O_NONBLOCK|O_CLOEXEC);
 }
 if(p->fd<0)return -1;
 unsigned long keys[(KEY_MAX+8*sizeof(long))/(8*sizeof(long))]={0},axes[(ABS_MAX+8*sizeof(long))/(8*sizeof(long))]={0};
 struct uinput_user_dev u={0};
 if(ioctl(source,EVIOCGNAME(sizeof(u.name)),u.name)<0||ioctl(source,EVIOCGID,&u.id)<0||
    ioctl(source,EVIOCGBIT(EV_KEY,sizeof(keys)),keys)<0||ioctl(source,EVIOCGBIT(EV_ABS,sizeof(axes)),axes)<0)return -1;
 if(p->canonical){
  snprintf(u.name,sizeof(u.name),"%s",p->gta?"GTASA-Gamepad":"GO-Super Gamepad");
  u.id=(struct input_id){.bustype=BUS_HOST,.vendor=0x484b,.product=0x1100,.version=0x100};
 }
 if(ioctl(p->fd,UI_SET_EVBIT,EV_KEY)<0||ioctl(p->fd,UI_SET_EVBIT,EV_ABS)<0)return -1;
 if(p->canonical){
  const int canonical_keys[]={304,305,307,308,310,311,312,313,544,545,546,547,704,705,706,707,708};
  for(unsigned k=0;k<sizeof(canonical_keys)/sizeof(canonical_keys[0]);k++)if(ioctl(p->fd,UI_SET_KEYBIT,canonical_keys[k])<0)return -1;
 }else for(int k=0;k<=KEY_MAX;k++)if(bit(keys,k)&&ioctl(p->fd,UI_SET_KEYBIT,k)<0)return -1;
 for(int k=0;k<=ABS_MAX;k++)if(p->canonical?(k==ABS_X||k==ABS_Y||k==ABS_RX||k==ABS_RY||k==ABS_HAT0X||k==ABS_HAT0Y):(bit(axes,k)||k==ABS_X||k==ABS_Y)) {
  struct input_absinfo a={0};ioctl(source,EVIOCGABS(k),&a);
  if(k==ABS_X||k==ABS_Y||(p->canonical&&(k==ABS_RX||k==ABS_RY))){a.minimum=-32767;a.maximum=32767;a.flat=0;a.fuzz=0;}
  if(ioctl(p->fd,UI_SET_ABSBIT,k)<0)return -1;
  u.absmin[k]=a.minimum;u.absmax[k]=a.maximum;u.absflat[k]=a.flat;u.absfuzz[k]=a.fuzz;
 }
 if(write(p->fd,&u,sizeof(u))!=sizeof(u)||ioctl(p->fd,UI_DEV_CREATE)<0)return -1;
 char sysname[80]={0},folder[PATH_MAX];
 if(ioctl(p->fd,UI_GET_SYSNAME(sizeof(sysname)),sysname)<0)return -1;
 snprintf(folder,sizeof(folder),"/sys/devices/virtual/input/%s",sysname);
 for(int retry=0;retry<100;retry++) {
  DIR *d=opendir(folder);struct dirent *e;
  if(d){while((e=readdir(d))) {
   if(!strncmp(e->d_name,"event",5))snprintf(p->event,sizeof(p->event),"/dev/input/%s",e->d_name);
   if(!strncmp(e->d_name,"js",2))snprintf(p->js,sizeof(p->js),"/dev/input/%s",e->d_name);
  }closedir(d);}
  if(p->event[0]&&p->js[0]&&access(p->event,F_OK)==0&&access(p->js,F_OK)==0)break;
  usleep(10000);
 }
 if(!p->event[0]||access(p->event,F_OK)<0)return -1;
 char original[PATH_MAX],link[80];snprintf(link,sizeof(link),"/proc/self/fd/%d",source);
 ssize_t len=readlink(link,original,sizeof(original)-1);if(len<=0)return -1;original[len]=0;
 if(ioctl(source,EVIOCGRAB,1)<0)return -1;
 /* Readers of the original node use its virtual replacement; direct opens of
  * the additional virtual node are hidden to avoid duplicate controllers. */
 setenv("SLOT_PORT_REAL_EVENT",original,1);setenv("SLOT_PORT_VIRTUAL_EVENT",p->event,1);
 const char *base=strrchr(original,'/');
 if(base){
  char physical[PATH_MAX];snprintf(physical,sizeof(physical),"/sys/class/input/%s/device",base+1);
  DIR *d=opendir(physical);struct dirent *e;
  if(d){while((e=readdir(d)))if(!strncmp(e->d_name,"js",2)) {
   char path[PATH_MAX];snprintf(path,sizeof(path),"/dev/input/%s",e->d_name);setenv("SLOT_PORT_REAL_JS",path,1);
  }closedir(d);}
 }
 if(p->js[0])setenv("SLOT_PORT_VIRTUAL_JS",p->js,1);
 struct input_absinfo a;
 if(ioctl(source,EVIOCGABS(ABS_HAT0X),&a)==0)p->x=a.value;
 if(ioctl(source,EVIOCGABS(ABS_HAT0Y),&a)==0)p->y=a.value;
 p->active=1;input_directions(p);
 fprintf(stderr,"slot ports: native pad %s -> %s mode=%s\n",original,p->event,p->mode?"left-stick":"dpad");return 1;
}
static void input_event(port_input *p,int source,const struct input_event *e) {
 if(!p->active)return;
 if(e->type==EV_KEY&&e->code==354)return; /* Menu echo, not a game key. */
 if(e->type==EV_KEY&&e->code==p->menu_code) {
  p->menu=e->value!=0;
  if(p->n64){input_emit(p,EV_KEY,708,p->menu);input_emit(p,EV_SYN,SYN_REPORT,0);}
  if(!p->menu&&!p->select&&!p->r3_down)p->chord=0;
  return;
 }
 if(e->type==EV_KEY&&e->code==310 /* SP physical Select; 314 is L2 */) {
  p->select=e->value!=0;
  if(e->value==1&&p->menu&&!p->chord) {
   p->chord=p->suppress_select=1;p->mode=!p->mode;
   if(input_save(p)<0)fprintf(stderr,"slot ports: input mode could not be saved\n");
   input_directions(p);fprintf(stderr,"slot ports: input mode=%s\n",p->mode?"left-stick":"dpad");
  }
  if(p->suppress_select){if(!p->select)p->suppress_select=0;if(!p->select&&!p->menu&&!p->r3_down)p->chord=0;return;}
 }
 if(p->nfs&&e->type==EV_KEY&&e->code==307){
  if(e->value==1&&p->menu){p->r3_down=p->chord=1;input_emit(p,EV_KEY,707,1);input_emit(p,EV_SYN,SYN_REPORT,0);return;}
  if(p->r3_down){if(e->value==0){p->r3_down=0;input_emit(p,EV_KEY,707,0);input_emit(p,EV_SYN,SYN_REPORT,0);if(!p->menu&&!p->select)p->chord=0;}return;}
 }
 if(p->n64&&p->menu&&e->type==EV_KEY&&(e->code==306||e->code==307)&&e->value==1)p->chord=1;
 if(source!=p->source)return;
 if(e->type==EV_SYN&&e->code==SYN_DROPPED){stopped=1;fprintf(stderr,"slot ports: input stream overrun; returning to Slot safely\n");return;}
 if(e->type==EV_ABS&&(e->code==ABS_HAT0X||e->code==ABS_HAT0Y)) {
  int v=e->value<0?-1:e->value>0?1:0;
  if(e->code==ABS_HAT0X)p->x=v;else p->y=v;
  if(p->canonical){input_directions(p);return;}
  input_emit(p,EV_ABS,p->mode?(e->code==ABS_HAT0X?ABS_X:ABS_Y):e->code,p->mode?v*32767:v);return;
 }
 /* These SP pads expose dummy stick axes. Only the D-pad may drive X/Y. */
 if(e->type==EV_ABS&&(p->canonical||e->code==ABS_X||e->code==ABS_Y))return;
 if(e->type==EV_KEY||e->type==EV_ABS||e->type==EV_SYN)input_emit(p,e->type,e->type==EV_KEY?input_key(p,e->code):e->code,e->value);
}
/* Mount aliases exist only in the game's mount namespace. The supervisor
 * retains the original device FD and never changes the host's device tree.
 * This also covers 32-bit and static ports without an ABI-specific preload. */
static int input_child(port_input *p) {
 if(!p->active||p->test)return 0;
 const char *real=getenv("SLOT_PORT_REAL_EVENT"),*virt=getenv("SLOT_PORT_VIRTUAL_EVENT");
 const char *rjs=getenv("SLOT_PORT_REAL_JS"),*vjs=getenv("SLOT_PORT_VIRTUAL_JS");
 if(!real||!virt||unshare(CLONE_NEWNS)<0||mount(NULL,"/",NULL,MS_REC|MS_PRIVATE,NULL)<0)return -1;
 if(mount(virt,real,NULL,MS_BIND,NULL)<0)return -1;
 if(rjs&&(!vjs||mount(vjs,rjs,NULL,MS_BIND,NULL)<0))return -1;
 if(mount("/dev/null",virt,NULL,MS_BIND,NULL)<0)return -1;
 if(vjs&&mount("/dev/null",vjs,NULL,MS_BIND,NULL)<0)return -1;
 return 0;
}
static void input_close(port_input *p,struct pollfd *fds) {
 if(p->fd>=0){if(!p->test)ioctl(p->fd,UI_DEV_DESTROY);close(p->fd);}
 if(p->active&&!p->test)ioctl(fds[p->source].fd,EVIOCGRAB,0);
}
