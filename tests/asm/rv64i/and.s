    .text
    .globl _start
_start:
    li t3, 0b0101
    li t4, 0b1010
    and  t5, t4, t3

    call exit
