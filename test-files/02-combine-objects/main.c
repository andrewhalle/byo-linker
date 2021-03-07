#include <stdio.h>

int next_factorial();

int main() {
  for (int i = 0; i < 10; i++) {
    printf("%d\n", next_factorial());
  }
}
