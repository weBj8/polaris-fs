/* Differential harness: drives charts (C reference or the Rust port, same
 * symbols) through a fixed scenario and writes every observable output to a
 * blob file. Built against target/ref/moosefs-ref/mfscommon/charts.h. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include <time.h>
#include "charts.h"
void mycrc32_init(void);

#define STRID(a,b,c,d) (((((uint8_t)a)*256U+(uint8_t)b)*256U+(uint8_t)c)*256U+(uint8_t)d)

static FILE *blob;
static void emit(const void *p, uint32_t l) { fwrite(&l,4,1,blob); fwrite(p,1,l,blob); }
static void emit32(uint32_t v) { emit(&v,4); }

int main(int argc,char **argv) {
	const char *dir = argv[1];
	const char *tag = argv[2];
	char path[512],out[512];
	FILE *f;
	uint8_t hdr[16] = {0,1,0,0, 0,0,0x10,0, 0,0,0,0, 0,0,0,0};
	uint32_t T = 29000000, k, off, t, i, r, w, n, m, e;
	uint64_t d[3];
	uint8_t *buf;
	static const uint32_t calcs[] = { CHARTS_CALCDEF(CHARTS_MAX(CHARTS_CONST(0),CHARTS_SUB(0,1))), CHARTS_CALCDEF(CHARTS_DIV(CHARTS_MUL(2,CHARTS_CONST(3)),CHARTS_NEG(CHARTS_ADD(0,CHARTS_MIN(1,2))))), CHARTS_DEFS_END };
	static const statdef stats[] = {
		{"alpha",STRID('A','A','A','A'),CHARTS_MODE_ADD,0,CHARTS_SCALE_NONE,1,1},
		{"beta",STRID('B','B','B','B'),CHARTS_MODE_MAX,1,CHARTS_SCALE_MILI,1000,60},
		{"gamma",STRID('C','C','C','C'),CHARTS_MODE_ADD,0,CHARTS_SCALE_MICRO,100,60},
		{NULL,0,0,0,0,0,0}
	};
	static const estatdef estats[] = {
		{NULL,STRID('E','E','E','E'),CHARTS_CALC(0),CHARTS_DIRECT(2),CHARTS_NONE,CHARTS_MODE_ADD,0,CHARTS_SCALE_NONE,1,1},
		{NULL,STRID('F','F','F','F'),CHARTS_DIRECT(0),CHARTS_DIRECT(1),CHARTS_DIRECT(2),CHARTS_MODE_MAX,1,CHARTS_SCALE_KILO,1,1},
		{NULL,STRID('G','G','G','G'),CHARTS_CALC(1),CHARTS_NONE,CHARTS_NONE,CHARTS_MODE_ADD,0,CHARTS_SCALE_GIGA,8,3},
		{NULL,0,0,0,0,0,0,0,0,0}
	};
	static const uint32_t types[] = {0,1,2,3,100,101,102,103};
	static const uint32_t sizes[][2] = {{0,0},{1000,120},{150,50},{4200,1100},{777,333}};
	static const uint32_t ids[] = {STRID('A','A','A','A'),STRID('A','A','A','a'),STRID('F','F','f','F'),STRID('F','f','f','f'),STRID('E','e','E','E'),STRID('Z','Z','Z','Z'),STRID('G','g','g','G')};
	static const uint32_t maxes[] = {0,1,100,4096,5000};
	static const uint32_t gets[] = {0,1,60,4096,4097};

	mycrc32_init();
	setenv("TZ",argc>3?argv[3]:"UTC0",1);
	tzset();
	snprintf(path,sizeof(path),"%s/%s.stats",dir,tag);
	hdr[12]=T>>24; hdr[13]=T>>16; hdr[14]=T>>8; hdr[15]=T;
	f = fopen(path,"wb"); fwrite(hdr,1,16,f); fclose(f);

	snprintf(out,sizeof(out),"%s/%s.blob",dir,tag);
	blob = fopen(out,"wb");
	emit32(charts_init(calcs,stats,estats,path,1));
	off = 0;
	for (k=0 ; k<6000 ; k++) {
		if (k==3000) off += 2*86400;
		t = T*60 + off + k*60 + (k%7)*3;
		if (k%250==100) t -= 1800;
		d[0] = (k*7919U)%100000;
		d[1] = (k*104729U)%5000 + ((k%3==0)?UINT64_C(100000000000000000):0);
		d[2] = (k==4000)?UINT64_C(3000000000000000000):(k*31U)%977;
		charts_add(d,t);
	}
	for (n=0 ; n<2 ; n++) {
		for (i=0 ; i<sizeof(types)/4 ; i++) for (r=0 ; r<5 ; r++) for (w=0 ; w<5 ; w++) {
			m = charts_make_png(types[i]*10+r,sizes[w][0],sizes[w][1]);
			buf = malloc(m); charts_get_png(buf); emit(buf,m); free(buf);
		}
		for (i=0 ; i<sizeof(ids)/4 ; i++) {
			m = charts_make_png(ids[i],0,0);
			buf = malloc(m); charts_get_png(buf); emit(buf,m); free(buf);
		}
		for (i=0 ; i<sizeof(types)/4+sizeof(ids)/4 ; i++) for (r=0 ; r<10 ; r++) for (w=0 ; w<5 ; w++) for (e=0 ; e<2 ; e++) {
			uint32_t id = (i<sizeof(types)/4)?types[i]*10+r:ids[i-sizeof(types)/4];
			m = charts_makedata(NULL,id,maxes[w],e);
			emit32(m);
			if (m>0) { buf = malloc(m); emit32(charts_makedata(buf,id,maxes[w],e)); emit(buf,m); free(buf); }
		}
		m = charts_monotonic_data(NULL); buf = malloc(m); charts_monotonic_data(buf); emit(buf,m); free(buf);
		for (i=0 ; i<4 ; i++) for (w=0 ; w<5 ; w++) { uint64_t g = charts_get(i,gets[w]); emit(&g,8); }
		emit32(charts_getmaxleng());
		for (i=0 ; i<sizeof(types)/4 ; i++) for (r=0 ; r<5 ; r++) {
			static double dd[4096]; uint32_t ts=0xAAAAAAAA,rs=0x55555555;
			memset(dd,0,sizeof(dd));
			charts_getdata(dd,&ts,&rs,types[i]*10+r);
			emit(&ts,4); emit(&rs,4); emit(dd,sizeof(dd));
		}
		charts_store();
		f = fopen(path,"rb"); fseek(f,0,SEEK_END); m = ftell(f); fseek(f,0,SEEK_SET);
		buf = malloc(m); if (fread(buf,1,m,f)!=m) return 1; fclose(f); emit(buf,m); free(buf);
		charts_term();
		emit32(charts_init(calcs,stats,estats,path,1));
		charts_add(NULL,T*60+off+6000*60+120);
	}
	charts_term();
	fclose(blob);
	return 0;
}
