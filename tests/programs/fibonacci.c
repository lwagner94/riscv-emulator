//
// Created by lukas on 26.01.21.
//
#include "common/common.h"

uint32_t fib(uint32_t n) {
    if (n < 3) {
        return 1;
    }

    uint32_t t1 = 1;
    uint32_t t2 = 1;
    uint32_t next = 0;

    for (uint32_t i = 1; i <= n; i++) {
        t1 = t2;
        t2 = next;
        next = t1 + t2;
    }

    return next;
}

int main(void) {
    uint8_t n = read_byte();
    uint32_t result = fib(n);
    write_word(result);
    return 0;
}