#include <assert.h>
#include <stdio.h>

int char_num(char charnum) {
  switch (charnum) {
  case '0':
    return 0;
  case '1':
    return 1;
  case '2':
    return 2;
  case '3':
    return 3;
  case '4':
    return 4;
  case '5':
    return 5;
  case '6':
    return 6;
  case '7':
    return 7;
  case '8':
    return 8;
  case '9':
    return 9;
  default:
    return -1;
  }
}

int main() {
  FILE *file = fopen("day1.txt", "r");
  assert(file);

  int dial = 50;
  int dial_match = 0;
  char ch;

  do {
    int idx = 0;
    char rotate_to;
    int num = 0;

    while ((ch = fgetc(file)) != '\n') {
      if (ch == EOF) {
        goto calculate_result;
      }

      if (idx == 0) {
        rotate_to = ch;
      } else {
        num = num * 10 + char_num(ch);
      }

      idx++;
    }

    if (rotate_to == 'R') {
      for (int i = 0; i < num; i++) {
        dial += 1;
        dial = dial % 100;
        if (dial == 0) {
          dial_match += 1;
        }
      }
    } else if (rotate_to == 'L') {
      for (int i = 0; i < num; i++) {
        dial -= 1;
        dial = dial % 100;
        if (dial == 0) {
          dial_match += 1;
        }
      }
    }
  } while (1);

calculate_result:
  printf("%d\n", dial_match);

  fclose(file);
  return 0;
}
