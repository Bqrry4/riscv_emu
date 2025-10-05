    .text
    .globl _start
_start:
    li t3, 3
    li t4, 2
    sub  t5, t3, t4

    call exit
