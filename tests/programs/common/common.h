#ifndef RISCV_EMU_COMMON_H
#define RISCV_EMU_COMMON_H

#include <stdint.h>
#include <stddef.h>

#define DEBUG_BASE (0x20000000)

#define DEBUG_BASE_OUTPUT_LENGTH (DEBUG_BASE + 1024)
#define DEBUG_BASE_OUTPUT (DEBUG_BASE_OUTPUT_LENGTH + 4)
#define DEBUG_BASE_INPUT_LENGTH (DEBUG_BASE + 2 * 1024)
#define DEBUG_BASE_INPUT (DEBUG_BASE_INPUT_LENGTH + 4)


void write_byte(uint8_t value);
void write_halfword(uint16_t value);
void write_word(uint32_t value);
void write_string(const char* str, size_t maxlen);

uint8_t read_byte();
uint16_t read_halfword();
uint32_t read_word();
void read_string(char* str, size_t maxlen);

#endif //RISCV_EMU_COMMON_H
