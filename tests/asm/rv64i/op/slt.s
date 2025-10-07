    .text
    .globl _start
_start:
    addi t3, zero, -366
    addi t4, zero, 366
    slt  t5, t3, t4
    slti t6, t3, 366

    call exit
