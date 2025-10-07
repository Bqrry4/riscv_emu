    .text
    .globl _start
_start:
    li s1, 3
    li s2, 2
    sub  s3, s1, s2
    subw s6, s1, s2

    call exit
