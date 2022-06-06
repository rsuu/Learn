#include <stdio.h>

#include "f.h"

typedef char      s8;
typedef short     s16;
typedef int       s32;
typedef long long s64;

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;


#define SIZEOF_ARRAY(x)  (sizeof(x) / sizeof((x)[0]))

int main() {
  unsigned char *toc_buff[] = {0,1,2,3,"awdawd",'a',9};
  unsigned char *p = toc_buff;
  unsigned long entries_len = 37;
  unsigned char *end = p + entries_len;


  printf("start-ptr: %p\n", p);
  printf("end-ptr: %p\n", end);
  printf("start-val: %d\n", p);
  printf("end-val: %d\n", end);

  while (p < end) {
    printf("    p: %d\n", p);
    *p++;
  }

    void *arr[] = {0,1,2,3,"awdawd",'a',9};

    printf("arr[0]: %d\n", arr[0]);
    printf("arr[1]: %d\n", arr[1]);
    printf("arr[2]: %d\n", arr[2]);
    printf("arr[3]: %d\n", arr[3]);
    printf("arr[4]: %s\n", arr[4]);
    printf("arr[5]: %c\n", arr[5]);
    printf("arr[6]: %d\n", arr[6]);




int a[4] = {0,0,0,0};
size_t n = SIZEOF_ARRAY(a);
printf("%d\n",n);
  fff();
}


