    .text
    .globl _start
_start:
    addi s1, zero, -1
    addi s2, zero, 1
    add  s3, s1, s2

    addiw s5, zero, -1
    addw  s6, s5, s2

    call exit
