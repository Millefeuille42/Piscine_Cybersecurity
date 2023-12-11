#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// also found these useless functions

void n(void) {
  puts("Nope. ");
  return;
}

void ww(void) {
  puts("Good job. ");
  puts("Please entrer key: ");
  puts("%23s. ");
  puts("delabere. ");
  puts("%s, ");
  return;
}

// skipping functions that just takes a lot of space
//  like xd, xxd, xxxd, xyxxd that were like 
/*
 * void xd(void) {
 * 	puts("super long lorem ipsum type string");
 * 	puts("less long lorem ipsum type string");
 * 	return;
 * }
*/

// useful code begins here

void ok(void) {
	puts("Good job.");
	return;
}

void no(void) {
	puts("Nope.");
	exit(1);
}

int main(void) {
	size_t strlen_ret;
	int atoi_ret;
	char is_in_bounds;
	char character[4];
	char input_buffer[24];
	char password_buffer[9];
	int index_input;
	int index_password;
	int scanf_ret;
	int index_input_copy;

	printf("Please enter key: ");
	scanf_ret = scanf("%23s", input_buffer);
	if (scanf_ret != 1) {
		no();
	}
	if (input_buffer[1] != '0') {
		no();
	}
	if (input_buffer[0] != '0') {
		no();
	}
	fflush(0);
	
	memset(password_buffer, 0, 9);
	password_buffer[0] = 'd';
	character[3] = '\0';
	index_input = 2;
	index_password = 1;
	while (1) {
		strlen_ret = strlen(password_buffer);
		index_input_copy = index_input;
		is_in_bounds = 0;
		if (strlen_ret < 8) {
			strlen_ret = strlen(input_buffer);
			is_in_bounds = index_input_copy < strlen_ret;
		}
		if (!is_in_bounds) break;
		character[0] = input_buffer[index_input];
		character[1] = input_buffer[index_input + 1];
		character[2] = input_buffer[index_input + 2];
		atoi_ret = atoi(character);
		password_buffer[index_password] = (char)atoi_ret;
		index_input += 3;
		index_password += 1;
	}
	password_buffer[index_password] = '\0';
	atoi_ret = strcmp(password_buffer,"delabere");
	if (atoi_ret == 0) {
		ok();
	}
	else {
		no();
	}
	return 0;
}
