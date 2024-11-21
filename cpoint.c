#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

// Build with gcc -o point cpoint.c
// Run with ./point
int main() {

	char lit[20] = "Hello world!";
	char* s = lit;
	printf("%s\n", s);
	printf("%c\n", s[6]);
	*(s+6) = 'W';
	printf("%s\n", s);

	// C++ code?
	// int x = 10;
	// int &r = x;
	// assert(r == 10);
	// r = 20;

	// C code!
	int x = 10;
	int *r = &x;
	printf("%p\n", r);
	assert(*r==10);
	*r = 20;
	printf("%d\n", *r);
	return 0;
}
