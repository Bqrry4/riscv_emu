    .text
    .globl _start
_start:
    li s1, 0b1001
    li s2, 0b0101
    xor  s3, s1, s2
    xori s4, s1, 0b0101

    call exit
