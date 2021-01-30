//
// Created by lukas on 26.01.21.
//
#include "common/common.h"

uint32_t fib(uint32_t n) {
    if (n == 1 || n == 2) {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

int main(void) {
    uint8_t n = read_byte();
    uint32_t result = fib(n);
    write_word(result);
    return 0;
}