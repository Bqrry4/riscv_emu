    .text
    .globl _start
_start:
    li t3, 1
    li t4, 0
    div t5, t3, t4

    call exit
