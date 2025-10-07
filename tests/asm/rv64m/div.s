    .text
    .globl _start
_start:
    li s1, -1
    li s2, 3
    div  s3, s1, s2
    divw s4, s1, s2

    call exit
