const fibonacci = `int fibonacci(int a) {
  if (a < 2) {
    return a;
  }
  return fibonacci(a - 1) + fibonacci(a - 2);
}
void main() {
  print(fibonacci(10));
}
`;

const bubbleSort = `void bubbleSort(int a[], int len) {
   int i = len - 1;
   while (i > 0) {
      int j = 0;
      while (j < i) {
         if (a[j] > a[j + 1]) {
            int tem = a[j];
            a[j] = a[j + 1];
            a[j + 1] = tem;
         }
         j = j + 1;
      }
      i = i- 1;
   }
}
void main() {
   int a[5];
   a[0] = 4;
   a[1] = 10;
   a[2] = 1;
   a[3] = 7;
   a[4] = 2;
   bubbleSort(a, 5);
   print(a);
}
`;

export default {
    fibonacci,
    bubbleSort
}
