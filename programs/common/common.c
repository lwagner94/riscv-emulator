//
// Created by lukas on 8/15/19.
//

#include "common.h"

#define SIZE_X (800)
#define SIZE_Y (600)
#define BYTE_PER_PIXEL (4)

volatile char* output = 0x20000000;
volatile char* FRAMEBUFFER_BASE = 0x40000000;

void debug(char* string) {
    char* ptr = string;

    while (*ptr) {
        *output = *ptr;
        ptr++;
    }
    return 0;
}

void draw_pixel(int x, int y, uint32_t color) {

    char* addr = FRAMEBUFFER_BASE + y * SIZE_X * BYTE_PER_PIXEL + x * BYTE_PER_PIXEL;
    *(uint32_t*)addr = color;
}

void clear_screen() {
    for (int y = 0; y < SIZE_Y; y++) {
        for (int x = 0; x < SIZE_X; x++) {
            draw_pixel(x, y, 0);
        }
    }
}

void draw_rect(int x_start, int y_start, int width, int height, uint32_t color) {
    for (int y = y_start; y < y_start + height; y++) {
        for (int x = x_start; x < x_start + width; x++) {
            draw_pixel(x, y, color);
        }
    }
}