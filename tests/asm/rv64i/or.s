    .text
    .globl _start
_start:
    li t3, 0b0101
    li t4, 0b1010
    or  t5, t3, t4
    ori t6, t3, 0b1010


    call exit
