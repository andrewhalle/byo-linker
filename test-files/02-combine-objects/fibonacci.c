int last2 = 0;
int last1 = 1;

int next_fibonacci () {
  int tmp = last2;
  last2 = last1;
  last1 = last2 + tmp;
  return tmp;
}
