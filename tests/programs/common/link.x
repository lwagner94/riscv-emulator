MEMORY
{
    ram : ORIGIN = 0x0, LENGTH = 0x100000
}

SECTIONS
{
    .text : { *(.text*) } > ram
    .rodata : { *(.rodata*) } > ram
    .bss : { *(.bss*) } > ram
}