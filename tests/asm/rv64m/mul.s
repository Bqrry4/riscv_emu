    .text
    .globl _start
_start:
    li t3, 2
    li t4, 2
    mul  t5, t3, t4

    call exit
