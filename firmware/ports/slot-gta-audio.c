/* MIT. GTA's wrapper replaces ALSOFT_CONF during startup. Keep its audio on
 * Slot's softvol PCM; do not intercept graphics, game input, or other ports. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int setenv(const char *name,const char *value,int overwrite) {
    static int (*next)(const char *,const char *,int);
    if(!next)next=dlsym(RTLD_NEXT,"setenv");
    const char *pinned=getenv("SLOT_GTA_ALSOFT_CONF");
    if(pinned && pinned[0]=='/' && !strcmp(name,"ALSOFT_CONF")) {
        static int logged;
        if(!logged){fprintf(stderr,"slot GTA audio: keeping Slot volume-controlled OpenAL output\n");logged=1;}
        return next(name,pinned,1);
    }
    return next(name,value,overwrite);
}
