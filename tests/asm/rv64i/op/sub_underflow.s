    .text
    .globl _start
_start:
    li s1, 0
    li s2, 1

    sub  s3, s1, s2
    subw s6, s1, s2

    call exit
