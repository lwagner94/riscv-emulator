//-------------------------------------------------------------------
//-------------------------------------------------------------------
void PUT32( unsigned int, unsigned int);
unsigned int GET32 ( unsigned int );
void  dummy ( unsigned int );
//-------------------------------------------------------------------

#include <string.h>
#include <stdint.h>
#include "../common/common.h"


void* end = 0x100000;



int notmain ( void )
{
    debug("Drawing");

    int x = 0;


    while (1) {
        draw_rect(x, 450, 1, 50, 0x00);
        x++;
        draw_rect(x, 450, 50, 50, 0xFFFFFF00);

        for (volatile int i = 0; i < 100000; i++) ;
    }
}
//-------------------------------------------------------------------
