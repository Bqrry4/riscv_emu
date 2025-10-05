    .text
    .globl _start
_start:
    li t3, 0
    li t4, 1
    sub  t5, t3, t4

    call exit
