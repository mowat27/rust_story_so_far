// main.c
#include <stdio.h>

int main(int argc, char **argv) {
  unsigned int a[3];

  for(int i=0; i<2; ++i) {
    a[i] = i;
  }

  for(int i=0; i<5; ++i) {
    printf("a[%d] = %d\n", i, a[i]);
  }
}