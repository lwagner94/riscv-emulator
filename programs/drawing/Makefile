RISCVGNU = riscv64-unknown-elf


AOPS = -march=rv32ima -g -mabi=ilp32
COPS = -march=rv32ima -g -mabi=ilp32 -Wall -O3 -nostdlib -nostartfiles -ffreestanding

INC = /home/lukas/Scrapspace/newlib-install/riscv64-unknown-elf/include


all : main.bin

clean :
	rm -f *.o
	rm -f *.elf
	rm -f *.bin
	rm -f *.list

novectors.o : ../common/novectors.s
	$(RISCVGNU)-as $(AOPS) ../common/novectors.s -o novectors.o


main.o : main.c
	$(RISCVGNU)-gcc $(COPS) -I$(INC) -c main.c -o main.o

common.o : ../common/common.c
	$(RISCVGNU)-gcc $(COPS) -I$(INC) -c ../common/common.c -o common.o

main.bin : ../common/memmap novectors.o main.o common.o
	$(RISCVGNU)-ld  -m elf32lriscv novectors.o main.o common.o ../common/libc.a ../common/libm.a ../common/libgcc.a -T ../common/memmap -o main.elf

	$(RISCVGNU)-objdump -D main.elf > main.list
	$(RISCVGNU)-objcopy main.elf -S -O binary main.bin
