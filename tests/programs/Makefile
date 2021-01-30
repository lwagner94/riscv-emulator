RISCVGNU = riscv64-elf


CFLAGS = -march=rv32im -g -mabi=ilp32 -Wall -O2 -nostartfiles -ffreestanding
LDFLAGS = -T common/link.x --specs=nosys.specs


FILES = $(wildcard *.c)
ELF_FILES = $(patsubst %.c, %.elf, $(FILES))


%.elf : %.c common/common.c common/common.h common/link.x Makefile
	$(RISCVGNU)-gcc $(CFLAGS) $(LDFLAGS) common/startup.s common/common.c $< -o $@
	#$(RISCVGNU)-objdump -D $@ > $@.list

all: $(ELF_FILES)
#
clean :
	rm -f *.elf
	rm -f *.list

