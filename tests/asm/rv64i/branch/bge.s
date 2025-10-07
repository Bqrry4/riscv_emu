    .text
    .globl _start
_start:
    li t3, 366
    li t4, -366

    bge t3, t4, true

    li t5, 0
    j done
true:
    li t5, 1
done:
    call exit
