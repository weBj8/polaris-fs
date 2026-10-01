#include <string.h>
void mfs_log(int mode,int priority,const char *fmt,...) { (void)mode; (void)priority; (void)fmt; }
const char* strerr(int e) { return strerror(e); }
