    .text
    .globl _start
_start:
    addi t3, zero, 1
    addi t4, zero, 2
    sll  t5, t3, t4
    slli t6, t3, 2

    call exit
