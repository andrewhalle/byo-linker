#include <stdio.h>

int next_fibonacci();

int main() {
  for (int i = 0; i < 10; i++) {
    printf("%d\n", next_fibonacci());
  }
}
