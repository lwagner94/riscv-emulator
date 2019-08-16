
#include <string.h>

#include "../common/common.h"



void* end = (void*) 0x100000;

volatile void* KEYBUFFER_BASE = 0x40200000;

char read_char();

volatile char current_char = 0;

void default_handler(void)__attribute__((interrupt));;
void default_handler(void) {
    current_char = read_char();
}

char read_char() {
    volatile unsigned int* ptr = ((volatile unsigned int*) KEYBUFFER_BASE);

    if (*ptr == 0) {
        return 0;
    }
    else {
        return *(ptr + 1);
    }
}



void notmain ( void )
{
    int x = 100;
    int y = 100;

    while (current_char != 'q') {
        int x_off = 0;
        int y_off = 0;

        switch (current_char) {
            case 'w': y_off = -5; break;
            case 's': y_off = 5; break;
            case 'a': x_off = -5; break;
            case 'd': x_off = 5; break;
            default: break;
        }
        draw_rect(x, y, 20, 20, 0x00000000);

        x = x + x_off;
        y = y + y_off;
        draw_rect(x, y, 20, 20, 0xFFFFFFFF);

        for (volatile int i = 0; i < 1000000; i++) ;
    }


}

