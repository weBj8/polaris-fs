/* Seeded fuzz pass over charts (C reference or Rust port, same symbols).
 * usage: fuzz <dir> <tag> <seed> */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include <time.h>
#include "charts.h"
void mycrc32_init(void);

#define STRID(a,b,c,d) (((((uint8_t)a)*256U+(uint8_t)b)*256U+(uint8_t)c)*256U+(uint8_t)d)

static FILE *blob;
static uint64_t rs;
static uint64_t rnd(void) { rs ^= rs<<13; rs ^= rs>>7; rs ^= rs<<17; return rs; }
static void emit(const void *p, uint32_t l) { fwrite(&l,4,1,blob); fwrite(p,1,l,blob); }
static void emit32(uint32_t v) { emit(&v,4); }
static void put32(uint8_t *p,uint32_t v) { p[0]=v>>24; p[1]=v>>16; p[2]=v>>8; p[3]=v; }

static uint64_t rval(void) {
	switch (rnd()%8) {
		case 0: return 0;
		case 1: return rnd()%10;
		case 2: return rnd()%1000;
		case 3: return rnd()%1000000;
		case 4: return rnd()%UINT64_C(1000000000000);
		case 5: return rnd()>>(rnd()%64);
		case 6: return UINT64_MAX - rnd()%3;
		default: return rnd()%UINT64_C(2000000000000000000);
	}
}

int main(int argc,char **argv) {
	char path[512],out[512];
	FILE *f;
	uint32_t i,k,n,m,t,T,r,iter;
	uint64_t d[4];
	uint8_t *buf;
	static uint32_t calcs[64];
	static statdef stats[5];
	static estatdef estats[6];
	static const char *names[] = {"s0","s1","s2","s3"};
	static double dd[4096];
	(void)argc;
	rs = strtoull(argv[3],NULL,10)*2654435761ULL+1;
	mycrc32_init();
	setenv("TZ","Europe/Warsaw",1);
	tzset();
	snprintf(path,sizeof(path),"%s/%s.fstats",argv[1],argv[2]);
	snprintf(out,sizeof(out),"%s/%s.fblob",argv[1],argv[2]);
	blob = fopen(out,"wb");

	for (iter=0 ; iter<6 ; iter++) {
		/* random definitions */
		for (i=0 ; i<4 ; i++) {
			stats[i].name = (char*)names[i];
			stats[i].statid = STRID('S','0'+i,'X','Y');
			stats[i].mode = rnd()%2;
			stats[i].percent = rnd()%2;
			stats[i].scale = rnd()%6;
			stats[i].multiplier = 1+rnd()%9000;
			stats[i].divisor = 1+rnd()%100;
		}
		memset(&stats[4],0,sizeof(statdef));
		k=0;
		calcs[k++]=0; calcs[k++]=CHARTS_OP_CONST; calcs[k++]=rnd()%1000; calcs[k++]=1000+1+rnd()%6; calcs[k++]=CHARTS_OP_END;
		calcs[k++]=rnd()%4; calcs[k++]=rnd()%4; calcs[k++]=1000+1+rnd()%6; calcs[k++]=CHARTS_OP_NEG; calcs[k++]=rnd()%4; calcs[k++]=CHARTS_OP_SUB; calcs[k++]=CHARTS_OP_END;
		calcs[k++]=rnd()%4; calcs[k++]=rnd()%4; calcs[k++]=CHARTS_OP_DIV; calcs[k++]=CHARTS_OP_END;
		calcs[k++]=CHARTS_DEFS_END;
		for (i=0 ; i<5 ; i++) {
			uint32_t srcs[3];
			for (n=0 ; n<3 ; n++) {
				switch (rnd()%3) {
					case 0: srcs[n] = CHARTS_DIRECT(rnd()%4); break;
					case 1: srcs[n] = CHARTS_CALC(rnd()%3); break;
					default: srcs[n] = (n==0)?CHARTS_DIRECT(0):CHARTS_NONE;
				}
			}
			/* keep series contiguous (C needs c1 before c2 before c3) */
			if (srcs[1]==CHARTS_NONE) srcs[2]=CHARTS_NONE;
			estats[i].name = NULL;
			estats[i].statid = STRID('E','0'+i,'Q','W');
			estats[i].c1src = srcs[0]; estats[i].c2src = srcs[1]; estats[i].c3src = srcs[2];
			estats[i].mode = rnd()%2; estats[i].percent = rnd()%2; estats[i].scale = rnd()%6;
			estats[i].multiplier = 1+rnd()%9000; estats[i].divisor = 1+rnd()%100;
		}
		memset(&estats[5],0,sizeof(estatdef));

		/* random stats file: short/long fleng, unknown and known names */
		T = 28000000 + rnd()%2000000;
		f = fopen(path,"wb");
		{
			uint8_t h[16]; uint32_t fleng = (iter%3==0)?100:(iter%3==1)?4096:5000, fch = 3;
			put32(h,(iter==5)?0x00020000:0x00010000); put32(h+4,fleng); put32(h+8,fch); put32(h+12,T);
			fwrite(h,1,16,f);
			for (n=0 ; n<fch ; n++) {
				char nm[100]; memset(nm,0,100);
				strcpy(nm,(n==1)?"unknown":names[(n*3+iter)%4]);
				fwrite(nm,1,100,f);
				for (k=0 ; k<4*fleng ; k++) { uint8_t b[8]; uint64_t v = (rnd()%5==0)?UINT64_MAX:rval(); for (m=0;m<8;m++) b[m]=v>>(56-8*m); fwrite(b,1,8,f); }
			}
		}
		fclose(f);
		emit32(charts_init(calcs,stats,estats,path,1));

		t = T*60;
		for (k=0 ; k<3000 ; k++) {
			t += (rnd()%10==0) ? rnd()%7200 : 60 - (rnd()%3==0 ? rnd()%400 : 0);
			for (i=0 ; i<4 ; i++) d[i] = rval();
			charts_add((rnd()%50==0)?NULL:d,t);
		}
		for (k=0 ; k<60 ; k++) {
			uint32_t ty = (rnd()%2)?rnd()%5:100+rnd()%6;
			uint32_t id = (rnd()%4==0)?(rnd()%2?STRID('S','0'+rnd()%4,'X','Y'):STRID('E','0'+rnd()%5,'Q','W'))^((uint32_t)(rnd()%16)*0x00202020&0x20202020):ty*10+rnd()%5;
			uint32_t w = (rnd()%5==0)?0:rnd()%4500, h = (rnd()%5==0)?0:rnd()%1200;
			m = charts_make_png(id,w,h);
			buf = malloc(m); charts_get_png(buf); emit(buf,m); free(buf);
			for (r=0 ; r<2 ; r++) {
				uint32_t me = rnd()%5000;
				m = charts_makedata(NULL,id,me,r);
				emit32(m);
				if (m>0) { buf = malloc(m); emit32(charts_makedata(buf,id,me,r)); emit(buf,m); free(buf); }
			}
			{ uint32_t ts=1,rsc=2; memset(dd,0,sizeof(dd)); charts_getdata(dd,&ts,&rsc,id); emit(&ts,4); emit(&rsc,4); emit(dd,sizeof(dd)); }
			{ uint64_t g = charts_get(rnd()%6,rnd()%5000); emit(&g,8); }
		}
		m = charts_monotonic_data(NULL); buf = malloc(m); charts_monotonic_data(buf); emit(buf,m); free(buf);
		charts_store();
		f = fopen(path,"rb"); fseek(f,0,SEEK_END); m = ftell(f); fseek(f,0,SEEK_SET);
		buf = malloc(m); if (fread(buf,1,m,f)!=m) return 1; fclose(f); emit(buf,m); free(buf);
		charts_term();
	}
	fclose(blob);
	return 0;
}
