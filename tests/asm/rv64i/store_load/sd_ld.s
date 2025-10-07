    .text
    .globl _start
_start:
    la s1, dword
    li s2, 0x0101010101010101
    sd s2, 4(s1)
    ld s3, 4(s1)

    call exit

    .data
dword:
    .space 8
