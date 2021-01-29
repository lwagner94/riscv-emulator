.globl _start
_start:
    lui x2, 0x8000
    jal main
    ebreak
    j .
