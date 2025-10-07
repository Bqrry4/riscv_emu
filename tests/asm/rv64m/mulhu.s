    .text
    .globl _start
_start:
    li s1, -1
    li s2, 3
    mulhu s3, s1, s2

    call exit
