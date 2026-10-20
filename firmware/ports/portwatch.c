#define _GNU_SOURCE
#include <linux/input.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <sys/prctl.h>
#include <signal.h>
#include <fcntl.h>
#include <poll.h>
#include <time.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include "port-volume.h"
static volatile sig_atomic_t stopped;
static void stop(int s){(void)s;stopped=1;}
static long ms(void){struct timespec t;clock_gettime(CLOCK_MONOTONIC,&t);return t.tv_sec*1000+t.tv_nsec/1000000;}
static int bit(unsigned long*b,int n){return (b[n/(8*sizeof(long))]>>(n%(8*sizeof(long))))&1;}
static int held(long start,long now){return start>0&&now-start>=1000;}
#include "port-input.h"
int main(int argc,char**argv){
 if(argc==2&&!strcmp(argv[1],"--self-test")){if(held(0,2000)||held(100,1099)||!held(100,1100))return 1;puts("PASS Menu hold threshold and release reset");return 0;}
 if(argc<2)return 2;
 const char*mc=getenv("SLOT_MENU_CODE");int menu=mc?atoi(mc):708;if(menu<1||menu>KEY_MAX)return 2;
 int command=1; struct pollfd fds[64];int n=0,has_menu=0;const char*test_volume=NULL;
 if(!strncmp(argv[1],"--test-fd=",10)){if(argc<3)return 2;int fd=atoi(argv[1]+10);fcntl(fd,F_SETFL,O_NONBLOCK);fds[n++]=(struct pollfd){fd,POLLIN,0};command=2;}
 if(command==2&&argc>3&&!strncmp(argv[2],"--test-volume=",14)){test_volume=argv[2]+14;command=3;}
 for(int i=0;command==1&&i<64;i++){char p[64];snprintf(p,sizeof(p),"/dev/input/event%d",i);int fd=open(p,O_RDONLY|O_NONBLOCK|O_CLOEXEC);if(fd<0)continue;unsigned long b[(KEY_MAX+8*sizeof(long))/(8*sizeof(long))]={0};ioctl(fd,EVIOCGBIT(EV_KEY,sizeof(b)),b);if(bit(b,menu)||bit(b,KEY_VOLUMEUP)||bit(b,KEY_VOLUMEDOWN)||input_pad(fd)){has_menu|=bit(b,menu);fds[n++]=(struct pollfd){fd,POLLIN,0};fprintf(stderr,"slot ports: input monitor %s\n",p);}else close(fd);}
 if(!n||(command==1&&!has_menu)){fprintf(stderr,"slot ports: no Menu device; refusing unquittable launch\n");return 3;}
 port_volume volume={0};const char*root=getenv("SLOT_ROOT");
 if(command==1||test_volume){if(test_volume)root=test_volume;if(!root||volume_init(&volume,root,test_volume)<0){fprintf(stderr,"slot ports: volume initialization failed; refusing uncontrolled-volume launch\n");volume_close(&volume);return 5;}}
 port_input input={0};
 const char *test_input=command!=1?getenv("SLOT_INPUT_TEST_OUTPUT"):NULL;
 if(input_init(&input,root,getenv("SLOT_PORT_ID"),fds,n,test_input)<0){
  fprintf(stderr,"slot ports: native input setup failed; returning without launching: %s\n",strerror(errno));
  input_close(&input,fds);volume_close(&volume);return 6;
 }
 signal(SIGTERM,stop);signal(SIGINT,stop);signal(SIGHUP,stop);
 pid_t pid=fork();if(pid<0)return 4;if(!pid){prctl(PR_SET_PDEATHSIG,SIGTERM);setpgid(0,0);if(input_child(&input)<0){perror("slot ports: isolated controller routing");_exit(6);}execvp(argv[command],argv+command);perror("port exec");_exit(127);}setpgid(pid,pid);
 long down[64]={0};int status=0,ended=0;
 while(!stopped){pid_t w=waitpid(pid,&status,WNOHANG);if(w==pid){ended=1;break;}if(w<0&&errno!=EINTR)break;poll(fds,n,25);for(int j=0;j<n;j++){struct input_event e;while(read(fds[j].fd,&e,sizeof(e))==sizeof(e)){long now=ms();volume_event(&volume,&e,now);input_event(&input,j,&e);if(input.chord)memset(down,0,sizeof(down));if(e.type==EV_KEY&&e.code==menu){if(e.value==1)down[j]=now;if(e.value==0)down[j]=0;}if(e.type==EV_SYN&&e.code==SYN_DROPPED){down[j]=0;volume_reset_keys(&volume);}}if(!input.chord&&held(down[j],ms()))stopped=1;}volume_tick(&volume,ms());}
 kill(-pid,SIGTERM);
 if(!ended){for(int i=0;i<40;i++){if(waitpid(pid,&status,WNOHANG)==pid){ended=1;break;}usleep(50000);}}
 kill(-pid,SIGKILL);if(!ended)waitpid(pid,&status,0);
 input_close(&input,fds);
 for(int j=0;j<n;j++)close(fds[j].fd);
 volume_close(&volume);
 fprintf(stderr,"slot ports: game ended status=%d menu_exit=%d\n",status,stopped!=0);
 return stopped?0:WIFEXITED(status)?WEXITSTATUS(status):128+WTERMSIG(status);
}
