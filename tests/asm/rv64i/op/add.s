    .text
    .globl _start
_start:
    addi s1, zero, 1
    addi s2, zero, 2
    add  s3, s1, s2

    addiw s5, zero, 2
    addw  s6, s5, s1

    call exit
