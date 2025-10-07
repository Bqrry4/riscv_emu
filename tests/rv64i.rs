mod helper;
use crate::helper::assert_xregs;
use risc_v::{
    components::{mmu::Size, registers::XRegisters, system_bus::DRAM_BASE},
    cpu::Cpu,
};

/* op */
define_test!(add, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 1),
            (XRegisters::t4, 2),
            (XRegisters::t5, 3),
        ],
    );
});
define_test!(sub, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 3),
            (XRegisters::t4, 2),
            (XRegisters::t5, 1),
        ],
    );
});
define_test!(add_overflow, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 1),
            (XRegisters::t5, 0),
        ],
    );
});
define_test!(sub_underflow, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 0),
            (XRegisters::t4, 1),
            (XRegisters::t5, u64::MAX),
        ],
    );
});
define_test!(sll, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 1),
            (XRegisters::t4, 2),
            (XRegisters::t5, 4),
            (XRegisters::t6, 4),
        ],
    );
});
define_test!(slt, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, (-366_i64) as u64),
            (XRegisters::t4, 366),
            (XRegisters::t5, 1),
            (XRegisters::t6, 1),
        ],
    );
});
define_test!(sltu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, (-366_i64) as u64),
            (XRegisters::t4, 366),
            (XRegisters::t5, 0),
            (XRegisters::t6, 0),
        ],
    );
});
define_test!(xor, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 0b1001),
            (XRegisters::t4, 0b0101),
            (XRegisters::t5, 0b1100),
            (XRegisters::t6, 0b1100),
        ],
    );
});
define_test!(sra, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 1),
            (XRegisters::t5, u64::MAX),
            (XRegisters::t6, u64::MAX),
        ],
    );
});
define_test!(srl, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 1),
            (XRegisters::t5, u64::MAX >> 1),
            (XRegisters::t6, u64::MAX >> 1),
        ],
    );
});
define_test!(or, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 0b0101),
            (XRegisters::t4, 0b1010),
            (XRegisters::t5, 0b1111),
            (XRegisters::t6, 0b1111),
        ],
    );
});
define_test!(and, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 0b0101),
            (XRegisters::t4, 0b1010),
            (XRegisters::t5, 0b0000),
            (XRegisters::t6, 0b0000),
        ],
    );
});
define_test!(sb_lb, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t4, 0x0101_0101_0101_0101),
            (XRegisters::t5, 0x01),
        ],
    );
});

define_test!(sh_lh, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t4, 0x0101_0101_0101_0101),
            (XRegisters::t5, 0x0101),
        ],
    );
});
define_test!(sw_lw, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t4, 0x0101_0101_0101_0101),
            (XRegisters::t5, 0x0101_0101),
        ],
    );
});
define_test!(sd_ld, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t4, 0x0101_0101_0101_0101),
            (XRegisters::t5, 0x0101_0101_0101_0101),
        ],
    );
});
define_test!(beq, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 366),
            (XRegisters::t4, 366),
            (XRegisters::t5, 1),
        ],
    );
});
define_test!(bne, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, -366_i64 as u64),
            (XRegisters::t4, 366),
            (XRegisters::t5, 1),
        ],
    );
});
define_test!(blt, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, -366_i64 as u64),
            (XRegisters::t4, 366),
            (XRegisters::t5, 1),
        ],
    );
});
define_test!(bltu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, -366_i64 as u64),
            (XRegisters::t4, 366),
            (XRegisters::t5, 0),
        ],
    );
});
define_test!(bge, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 366),
            (XRegisters::t4, -366_i64 as u64),
            (XRegisters::t5, 1),
        ],
    );
});
define_test!(bgeu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 366),
            (XRegisters::t4, -366_i64 as u64),
            (XRegisters::t5, 0),
        ],
    );
});
