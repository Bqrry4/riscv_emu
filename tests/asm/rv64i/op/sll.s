    .text
    .globl _start
_start:
    addi s1, zero, 1
    addi s2, zero, 2

    sll  s3, s1, s2
    slli s4, s1, 2
    sllw s6, s1, s2
    slliw s7, s1, 2

    call exit
