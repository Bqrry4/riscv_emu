    .text
    .globl _start
_start:
    la t3, dword
    li t4, 0x0101010101010101
    sh t4, 0(t3)
    lh t5, 0(t3)

    call exit

    .data
dword:
    .space 8
