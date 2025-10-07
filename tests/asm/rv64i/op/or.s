    .text
    .globl _start
_start:
    li s1, 0b0101
    li s2, 0b1010
    or  s3, s1, s2
    ori s4, s1, 0b1010

    call exit
