#include <string.h>
#include <stdint.h>
#include "../common/common.h"
#include "font8x8_basic.h"



void* end = 0x100000;

void default_handler(void)__attribute__((interrupt));;
void default_handler(void) {
}

void render(char *bitmap, int x_offset, int y_offset, int scale) {
    int x,y;
    int set;
    int mask;
    for (x=0; x < 8 * scale; x++) {
        for (y=0; y < 8 * scale; y++) {
            set = bitmap[y / scale] & 1 << x / scale;
            if (set) {
                draw_pixel(x + x_offset, y + y_offset, 0xFFFFFFFF);
            }
            else {
                draw_pixel(x + x_offset, y + y_offset, 0x00000000);
            }

        }
    }
}

void render_string(char* str, int x_offset, int y_offset, int scale) {
    int len = strlen(str);

    for (int i = 0; i < len; i++) {
        char *bitmap = font8x8_basic[str[i]];
        render(bitmap, x_offset + i * 8 * scale, y_offset, scale);
    }
}


int notmain ( void )
{
    debug("Drawing");

    int x = 0;

    char *bitmap = font8x8_basic[65];
    render_string("root@host:/usr/local/bin:$", 20, 100, 2);


    while (1) {

    }
}
//-------------------------------------------------------------------
