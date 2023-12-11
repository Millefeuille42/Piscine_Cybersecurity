#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// also found these useless functions

void easy(void) {
  puts("easy.");
  return;
}

void it(void) {
  puts("it");
  return;
}

int nice(int val) {
  int ret;
  
  ret = puts("nice");
  return ret;
}

void that(void) {
  puts("that.");
  return;
}

void this(void) {
  puts("this");
  return;
}

void try(void) {
  puts("try");
  return;
}

void wt(void) {
  puts("********");
  return;
}

void not(void) {
  puts("not.");
  return;
}


// This was actually named ___syscall_malloc, for obfuscation purposes?
//  but I deliberatly renamed it here to avoid confusion
void no(void) {
  puts("Nope.");
  exit(1);
}

// Same but ____syscall_malloc
void ok(void) {
  puts("Good job.");
  return;
}

int main(void) {
  int atoi_ret;
  size_t strlen_ret;
  char is_in_bounds;
  char character [4];
  char input_buffer [31];
  char password_buffer [9];
  long buffer_index;
  int strcmp_ret;
  int password_index;
  int scanf_ret;
  long buffer_index_copy;
  
  printf("Please enter key: ");
  scanf_ret = scanf("%23s", input_buffer);
  if (scanf_ret != 1) {
    no();
  }
  if (input_buffer[1] != '2') {
    no();
  }
  if (input_buffer[0] != '4') {
    no();
  }
  fflush(0);
  memset(password_buffer,0,9);
  password_buffer[0] = '*';
  character[3] = '\0';
  buffer_index = 2;
  password_index = 1;
  while (1) {
    strlen_ret = strlen(password_buffer);
    buffer_index_copy = buffer_index;
    is_in_bounds = 0;
    if (strlen_ret < 8) {
      strlen_ret = strlen(input_buffer);
      is_in_bounds = buffer_index_copy < strlen_ret;
    }
    if (!is_in_bounds) break;
    character[0] = input_buffer[buffer_index];
    character[1] = input_buffer[buffer_index + 1];
    character[2] = input_buffer[buffer_index + 2];
    atoi_ret = atoi(character);
    password_buffer[password_index] = (char)atoi_ret;
    buffer_index = buffer_index + 3;
    password_index = password_index + 1;
  }
  password_buffer[password_index] = '\0';
  strcmp_ret = strcmp(password_buffer,"********");

  // This looks stupid, but these jumps were actually present in the assembly code...
  if (strcmp_ret == -2) {
    no();
  }
  else if (strcmp_ret == -1) {
    no();
  }
  else if (strcmp_ret == 0) {
    ok();
  }
  else if (strcmp_ret == 1) {
    no();
  }
  else if (strcmp_ret == 2) {
    no();
  }
  else if (strcmp_ret == 3) {
    no();
  }
  else if (strcmp_ret == 4) {
    no();
  }
  else if (strcmp_ret == 5) {
    no();
  }
  else if (strcmp_ret == 0x73) {
    no();
  }
  else {
    no();
  }
  return 0;
}
