/* Keep the G350 Menu monitor visible when a native port requests input grabs. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <sys/ioctl.h>
#include <linux/input.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
int ioctl(int fd,unsigned long request,...) {
    static int (*next)(int,unsigned long,...);
    if(!next) next=dlsym(RTLD_NEXT,"ioctl");
    va_list args;va_start(args,request);uintptr_t arg=va_arg(args,uintptr_t);va_end(args);
    if(request==EVIOCGRAB) {
        unsigned long keys[(KEY_MAX+8*sizeof(long))/(8*sizeof(long))]={0};
        if(next(fd,EVIOCGBIT(EV_KEY,sizeof(keys)),keys)>=0 &&
           (((keys[708/(8*sizeof(long))]>>(708%(8*sizeof(long))))&1)||
            ((keys[KEY_VOLUMEUP/(8*sizeof(long))]>>(KEY_VOLUMEUP%(8*sizeof(long))))&1)||
            ((keys[KEY_VOLUMEDOWN/(8*sizeof(long))]>>(KEY_VOLUMEDOWN%(8*sizeof(long))))&1))) {
            fprintf(stderr,"slot ports: kept G350 control input shared\n");return 0;
        }
    }
    return next(fd,request,arg);
}
