#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

void slot_core_log(unsigned level, const char *format, ...) {
    if (!format || (level == 0 && !getenv("SLOT_CORE_DEBUG"))) return;
    va_list args;
    va_start(args, format);
    fprintf(stderr, "slot: core[%u]: ", level);
    vfprintf(stderr, format, args);
    va_end(args);
    fflush(stderr);
}
