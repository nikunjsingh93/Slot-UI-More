"""Actual private mount-namespace test. Run in a disposable privileged container.
Uses temporary ordinary files; no physical input devices or SD cards.
"""
from pathlib import Path
import subprocess,tempfile
source=Path(__file__).resolve().parent
with tempfile.TemporaryDirectory() as tmp:
 p=Path(tmp);c=p/'namespace.c';exe=p/'namespace'
 c.write_text(r'''
#define _GNU_SOURCE
#include <linux/input.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <poll.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <assert.h>
static int stopped;
static int bit(unsigned long *b,int n){return (b[n/(8*sizeof(long))]>>(n%(8*sizeof(long))))&1;}
#include "port-input.h"
static void check(const char *path,const char *expected){
 char b[64]={0};int fd=open(path,O_RDONLY);assert(fd>=0);
 ssize_t n=read(fd,b,sizeof(b)-1);assert(n>=0);close(fd);assert(!strcmp(b,expected));
}
int main(int argc,char **argv){
 assert(argc==5);setenv("SLOT_PORT_REAL_EVENT",argv[1],1);setenv("SLOT_PORT_VIRTUAL_EVENT",argv[2],1);
 setenv("SLOT_PORT_REAL_JS",argv[3],1);setenv("SLOT_PORT_VIRTUAL_JS",argv[4],1);
 int original=open(argv[1],O_RDONLY);assert(original>=0);
 pid_t child=fork();assert(child>=0);
 if(!child){port_input p={.active=1};assert(input_child(&p)==0);
  check(argv[1],"virtual-event");check(argv[2],"");check(argv[3],"virtual-js");check(argv[4],"");
  char b[64]={0};assert(read(original,b,63)>0);assert(!strcmp(b,"physical-event"));_exit(0);
 }
 int status;assert(waitpid(child,&status,0)==child);assert(WIFEXITED(status)&&WEXITSTATUS(status)==0);
 check(argv[1],"physical-event");check(argv[2],"virtual-event");check(argv[3],"physical-js");check(argv[4],"virtual-js");close(original);
 puts("PASS real kernel namespace: evdev/joydev aliases, duplicate hiding, retained physical FD, automatic mount cleanup and unchanged host paths");
}''')
 subprocess.run(['gcc','-O2','-I',str(source),str(c),'-o',str(exe)],check=True)
 files=[]
 for name in ['physical-event','virtual-event','physical-js','virtual-js']:
  f=p/name;f.write_text(name);files.append(str(f))
 subprocess.run([str(exe),*files],check=True)
