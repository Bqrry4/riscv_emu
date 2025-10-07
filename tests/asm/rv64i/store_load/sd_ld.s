    .text
    .globl _start
_start:
    la t3, dword
    li t4, 0x0101010101010101
    sd t4, 0(t3)
    ld t5, 0(t3)

    call exit

    .data
dword:
    .space 8
