    .text
    .globl _start
_start:
    li s1, 0b0101
    li s2, 0b1010
    and  s3, s1, s2
    andi s4, s1, 0b1010

    call exit
