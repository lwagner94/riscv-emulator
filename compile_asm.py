
from sh import riscv64_unknown_elf_gcc as gcc
from sh import riscv64_unknown_elf_objdump as objdump
from sh import rm

import re


gcc("test.S", "-c", "-o", "test.o",  "-march=rv32ima", "-mabi=ilp32")
for line in objdump("-d", "test.o"):
    m = re.match(r"^\s+([0-9a-f]+):\s+([0-9a-f]+)\s+(.*)$", line)
    if m:
        offset = int(m.group(1), 16)
        code = int(m.group(2), 16)
        assembly = m.group(3)

        bin_string = ""

        for index, bin_digit in enumerate("{:032b}".format(code)[::-1]):

            if index % 4 == 0 and index != 0:
                bin_string += "_"
            bin_string += bin_digit

        bin_string = bin_string[::-1]

        d = "{:032b}".format(code)[::-1]

        r_type = "_".join((d[0:7], d[7:12], d[12:15], d[15:20], d[20:25], d[25:32]))[::-1]
        u_type = "_".join((d[0:7], d[7:12], d[12:32]))[::-1]

        print("r: 0b{} u: 0b{} {}".format(r_type, u_type, assembly))


rm("test.o")
