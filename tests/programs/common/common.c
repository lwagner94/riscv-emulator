#include "common.h"

static uint32_t write_offset = 0;
static uint32_t read_offset = 0;

//static const uint32_t* write_len = (uint32_t*) DEBUG_BASE_OUTPUT_LENGTH;
//static const uint32_t* const read_len = (uint32_t*) DEBUG_BASE_INPUT_LENGTH;



void write_byte(uint8_t value) {
    *((uint8_t*) (DEBUG_BASE_OUTPUT + write_offset)) = value;
    write_offset += 1;
}

void write_halfword(uint16_t value) {
    *((uint16_t*) (DEBUG_BASE_OUTPUT + write_offset)) = value;
    write_offset += 2;
}

void write_word(uint32_t value) {
    *((uint32_t*) (DEBUG_BASE_OUTPUT + write_offset)) = value;
    write_offset += 4;
}

void write_string(const char* str, size_t maxlen) {
    int ctr = 0;

    do {
        write_byte(*str);
        str++;
        ctr++;
    }
    while (*str && ctr < maxlen);
}

uint8_t read_byte() {
    uint8_t value = *((uint8_t*) (DEBUG_BASE_INPUT + read_offset));
    read_offset += 1;
    return value;
}

uint16_t read_halfword() {
    uint16_t value = *((uint16_t*) (DEBUG_BASE_INPUT + read_offset));
    read_offset += 2;
    return value;
}

uint32_t read_word() {
    uint32_t value = *((uint32_t*) (DEBUG_BASE_INPUT + read_offset));
    read_offset += 4;
    return value;
}

void read_string(char* str, size_t maxlen) {
    char c = 0;
    int ctr = 0;

    do {
        char c = read_byte();
        str[ctr] = c;
        ctr++;
    } while (c != 0 && ctr < maxlen);

}