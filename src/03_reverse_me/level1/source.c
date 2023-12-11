#include <stdio.h>
#include <string.h>

int main(void) {
	int cmp_result = 0;
	char password[14] = "__stack_check"; 
	char input_buffer[100];

	printf("Please enter key: ");
	scanf("%s", input_buffer);

	cmp_result = strcmp(input_buffer, password);
	if (cmp_result == 0) {
		printf("Good job.\n");
	} else {
		printf("Nope.\n");
	}
}
