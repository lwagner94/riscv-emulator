//
// Created by lukas on 26.01.21.
//
#include "common/common.h"

int main(void)
{
    char buffer[100];
    write_byte(read_byte());
    read_string(buffer, 100);
    write_string(buffer, 100);
    write_halfword(read_halfword());
    write_word(read_word());
    write_word(read_word());
    write_word(read_word());
    write_word(read_word());
    return 0;
}