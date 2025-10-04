    .equ ADDR, 0x100000
    .equ EXIT_VAL,  0x5555

    .globl exit
    # Writes FINISHER_PASS to the TEST device
exit:

    lui  t0, %hi(ADDR)
    addi t0, t0, %lo(ADDR)
    li   t1, EXIT_VAL
    sw   t1, 0(t0)

    ret
