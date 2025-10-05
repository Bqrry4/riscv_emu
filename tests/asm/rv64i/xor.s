    .text
    .globl _start
_start:
    li t3, 0b1001
    li t4, 0b0101
    xor  t5, t4, t3

    call exit
