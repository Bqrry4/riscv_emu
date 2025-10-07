    .text
    .globl _start
_start:
    li s1, -1
    li s2, 1

    sra  s3, s1, s2
    srai s4, s1, 1
    sraw s6, s1, s2
    sraiw s7, s1, 1

    call exit
