#ifndef RISCV_EMU_COMMON_H
#define RISCV_EMU_COMMON_H

#include <stdint.h>

extern void debug(char* string);

extern void draw_pixel(int x, int y, uint32_t color);

extern void clear_screen();

extern void draw_rect(int x_start, int y_start, int width, int height, uint32_t color);

#endif //RISCV_EMU_COMMON_H
