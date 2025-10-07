    .text
    .globl _start
_start:
    addi s1, zero, -366
    addi s2, zero, 366
    slt  s3, s1, s2
    slti s4, s1, 366

    call exit
