    .text
    .globl _start
_start:
    li t3, 0b1001
    li t4, 0b0101
    xor  t5, t3, t4
    xori t6, t3, 0b0101

    call exit
