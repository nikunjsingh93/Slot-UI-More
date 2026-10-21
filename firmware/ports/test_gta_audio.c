#define _GNU_SOURCE
#include <assert.h>
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
int main(int argc,char **argv) {
 assert(argc==3);
 setenv("UNRELATED_SETTING","kept",1);assert(!strcmp(getenv("UNRELATED_SETTING"),"kept"));
 setenv("ALSOFT_CONF",argv[2],1);assert(!strcmp(getenv("ALSOFT_CONF"),argv[1]));
 setenv("ALSOFT_CONF",argv[2],0);assert(!strcmp(getenv("ALSOFT_CONF"),argv[1]));
 void *lib=dlopen("libopenal.so.1",RTLD_NOW);assert(lib);
 void *(*open_device)(const char *)=dlsym(lib,"alcOpenDevice");
 void *(*create_context)(void *,const int *)=dlsym(lib,"alcCreateContext");
 int (*make_current)(void *)=dlsym(lib,"alcMakeContextCurrent");
 void (*destroy_context)(void *)=dlsym(lib,"alcDestroyContext");
 int (*close_device)(void *)=dlsym(lib,"alcCloseDevice");
 void *device=open_device(NULL);assert(device);void *context=create_context(device,NULL);assert(context);assert(make_current(context));
 usleep(100000);make_current(NULL);destroy_context(context);assert(close_device(device));
 puts("PASS GTA audio config pin: wrapper override blocked, unrelated environment preserved, real OpenAL/ALSA output opened");
}
