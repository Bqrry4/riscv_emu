    .text
    .globl _start
_start:
    li s1, -366
    li s2, 366

    bne s1, s2, true

    li s3, 0
    j done
true:
    li s3, 1
done:
    call exit
