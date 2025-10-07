    .text
    .globl _start
_start:
    li s1, -1
    li s2, 4
    rem s3, s1, s2
    remw s4, s1, s2

    call exit
