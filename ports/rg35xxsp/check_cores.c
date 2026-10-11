#include <dlfcn.h>
#include <stdio.h>
#include <string.h>
struct info { const char *name, *version, *extensions; unsigned char fullpath, extract; };
int main(int argc, char **argv) {
    for (int i=1; i<argc; ++i) {
        void *h=dlopen(argv[i], RTLD_NOW|RTLD_LOCAL);
        if (!h) { fprintf(stderr,"FAIL %s: %s\n",argv[i],dlerror()); return 1; }
        const char *symbols[]={"retro_api_version","retro_get_system_info","retro_init","retro_run","retro_load_game","retro_serialize","retro_unserialize"};
        for (unsigned s=0;s<sizeof(symbols)/sizeof(*symbols);s++) {
            if (!dlsym(h,symbols[s])) { fprintf(stderr,"FAIL %s: missing %s\n",argv[i],symbols[s]); return 1; }
        }
        unsigned (*api)(void)=dlsym(h,"retro_api_version");
        void (*getinfo)(struct info *)=dlsym(h,"retro_get_system_info");
        struct info x={0}; getinfo(&x);
        if (api()!=1 || !x.name || !x.extensions) return 1;
        printf("PASS %s: %s %s, extensions %s\n",argv[i],x.name,x.version,x.extensions);
        dlclose(h);
    }
    return 0;
}
