RISCVGNU = riscv64-unknown-elf


AOPS = -march=rv32ima -g -mabi=ilp32
COPS = -march=rv32ima -g -mabi=ilp32 -Wall -O3 -nostdlib -nostartfiles -ffreestanding

INC = /home/lukas/Scrapspace/newlib-install/riscv64-unknown-elf/include


all : notmain.bin flightpath.bin

clean :
	rm -f *.o
	rm -f *.elf
	rm -f *.bin
	rm -f *.list

novectors.o : novectors.s
	$(RISCVGNU)-as $(AOPS) novectors.s -o novectors.o

notmain.o : notmain.c
	$(RISCVGNU)-gcc $(COPS) -I$(INC) -c notmain.c -o notmain.o

flightpath.o : flightpath.c
	$(RISCVGNU)-gcc $(COPS) -I$(INC) -c flightpath.c -o flightpath.o

notmain.bin : memmap novectors.o notmain.o
	#$(RISCVGNU)-gcc $(COPS) -Xlinker -T -Xlinker memmap -lc novectors.o  notmain.o -o notmain.elf
	$(RISCVGNU)-ld  -m elf32lriscv novectors.o notmain.o libc.a libm.a libgcc.a -T memmap -o notmain.elf

	$(RISCVGNU)-objdump -D notmain.elf > notmain.list
	$(RISCVGNU)-objcopy notmain.elf -S -O binary notmain.bin

flightpath.bin : memmap novectors.o flightpath.o
	#$(RISCVGNU)-gcc $(COPS) -Xlinker -T -Xlinker memmap -lc novectors.o  notmain.o -o notmain.elf
	$(RISCVGNU)-ld  -m elf32lriscv novectors.o flightpath.o libc.a libm.a libgcc.a -T memmap -o flightpath.elf

	$(RISCVGNU)-objdump -D flightpath.elf > flightpath.list
	$(RISCVGNU)-objcopy flightpath.elf -S -O binary flightpath.bin
