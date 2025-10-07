    .text
    .globl _start
_start:
    li t3, -1
    li t4, 1
    sra  t5, t3, t4
    srai t6, t3, 1

    call exit
