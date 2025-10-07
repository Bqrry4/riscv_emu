    .text
    .globl _start
_start:
    la t3, dword
    li t4, 0x0101010101010101
    sb t4, 4(t3)
    lb t5, 4(t3)

    call exit

    .data
dword:
    .space 8
