//-------------------------------------------------------------------
//-------------------------------------------------------------------
void PUT32( unsigned int, unsigned int);
unsigned int GET32 ( unsigned int );
void  dummy ( unsigned int );
//-------------------------------------------------------------------


#define SIZE_X (800)
#define SIZE_Y (600)
#define BYTE_PER_PIXEL (3)

void* end = 0x100000;

volatile char* output = 0x20000000;
volatile char* FRAMEBUFFER_BASE = 0x40000000;


void default_handler(void)__attribute__((interrupt));;
void default_handler(void) {
    *output = 55;
    *FRAMEBUFFER_BASE = 100;
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

void printString(char* string) {
    char* ptr = string;

    while (*ptr) {
        *output = *ptr;
        ptr++;
    }
}

void draw_pixel(int x, int y, char r, char g, char b) {

    char* addr = FRAMEBUFFER_BASE + y * SIZE_X * BYTE_PER_PIXEL + x * BYTE_PER_PIXEL;
    *addr++ = r;
    *addr++ = g;
    *addr++ = b;
}

int notmain ( void )
{
    char buffer[100];
    printString("Hello World\n");


    int result = fib(35);

    itoa(result, buffer);
    printString(buffer);
}
//-------------------------------------------------------------------
