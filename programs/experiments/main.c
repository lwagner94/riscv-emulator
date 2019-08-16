
#include <string.h>

#include "../common/common.h"



void* end = 0x100000;



void default_handler(void)__attribute__((interrupt));;
void default_handler(void) {
}

void reverse(char s[]);

void itoa(int n, char s[])
{
    int i, sign;

    if ((sign = n) < 0)  /* record sign */
        n = -n;          /* make n positive */
    i = 0;
    do {       /* generate digits in reverse order */
        s[i++] = n % 10 + '0';   /* get next digit */
    } while ((n /= 10) > 0);     /* delete it */
    if (sign < 0)
        s[i++] = '-';
    s[i] = '\0';
    reverse(s);
}

void reverse(char s[])
{
    int i, j;
    char c;

    for (i = 0, j = strlen(s)-1; i<j; i++, j--) {
        c = s[i];
        s[i] = s[j];
        s[j] = c;
    }
}


int fib(int n){
    if ( n == 0){
        return 0;
    }
    if ( n == 1){
        return 1;
    }
    if ( n > 1) {
        return fib(n-1)+fib(n-2);
    }
    return 0;
}



int notmain ( void )
{
    char buffer[100];

    debug("Hello World\n");
    
    int result = fib(40);

    itoa(result, buffer);
    debug(buffer);
}

