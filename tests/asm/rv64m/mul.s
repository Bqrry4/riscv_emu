    .text
    .globl _start
_start:
    li s1, 2
    li s2, 2
    mul   s3, s1, s2
    mulw  s4, s1, s2

    call exit
