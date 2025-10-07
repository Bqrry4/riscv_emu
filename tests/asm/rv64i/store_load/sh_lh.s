    .text
    .globl _start
_start:
    la s1, dword
    li s2, 0x0101010101010101
    sh s2, 4(s1)
    lh s3, 4(s1)

    call exit

    .data
dword:
    .space 8
