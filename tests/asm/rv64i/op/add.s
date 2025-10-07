    .text
    .globl _start
_start:
    addi t3, zero, 1
    addi t4, zero, 2
    add  t5, t4, t3

    call exit
