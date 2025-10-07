    .text
    .globl _start
_start:
    li s1, -1
    li s2, 1

    srl  s3, s1, s2
    srli s4, s1, 1
    srlw s6, s1, s2
    srliw s7, s1, 1

    call exit
