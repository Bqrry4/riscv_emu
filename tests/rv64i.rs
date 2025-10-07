mod helper;
use crate::helper::assert_xregs;
use risc_v::{
    components::{registers::XRegisters, system_bus::DRAM_BASE},
    cpu::Cpu,
};

/* @Note for op instructions:
 * s1-s2: dword input
 * s3: dword output
 * s4: dword+imm output
 * s5: word input
 * s6: word output
 * s7: word+imm output
 * */

define_test!(add, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 1),
            (XRegisters::s2, 2),
            (XRegisters::s3, 3),
            (XRegisters::s5, 2),
            (XRegisters::s6, 3),
        ],
    );
});
define_test!(add_overflow, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 1),
            (XRegisters::s3, 0),
            (XRegisters::s5, u32::MAX as u64),
            (XRegisters::s6, 0),
        ],
    );
});
define_test!(sub, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 3),
            (XRegisters::s2, 2),
            (XRegisters::s3, 1),
            (XRegisters::s6, 1),
        ],
    );
});
define_test!(sub_underflow, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 0),
            (XRegisters::s2, 1),
            (XRegisters::s3, u64::MAX),
            (XRegisters::s6, u32::MAX as u64),
        ],
    );
});
define_test!(sll, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 1),
            (XRegisters::s2, 2),
            (XRegisters::s3, 4),
            (XRegisters::s4, 4),
            (XRegisters::s6, 4),
            (XRegisters::s7, 4),
        ],
    );
});
define_test!(slt, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, (-366_i64) as u64),
            (XRegisters::s2, 366),
            (XRegisters::s3, 1),
            (XRegisters::s4, 1),
        ],
    );
});
define_test!(sltu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, (-366_i64) as u64),
            (XRegisters::s2, 366),
            (XRegisters::s3, 0),
            (XRegisters::s4, 0),
        ],
    );
});
define_test!(xor, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 0b1001),
            (XRegisters::s2, 0b0101),
            (XRegisters::s3, 0b1100),
            (XRegisters::s4, 0b1100),
        ],
    );
});
define_test!(sra, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 1),
            (XRegisters::s3, u64::MAX),
            (XRegisters::s4, u64::MAX),
            (XRegisters::s6, u32::MAX as u64),
            (XRegisters::s7, u32::MAX as u64),
        ],
    );
});
define_test!(srl, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 1),
            (XRegisters::s3, u64::MAX >> 1),
            (XRegisters::s4, u64::MAX >> 1),
            (XRegisters::s6, (u32::MAX >> 1) as u64),
            (XRegisters::s7, (u32::MAX >> 1) as u64),
        ],
    );
});
define_test!(or, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 0b0101),
            (XRegisters::s2, 0b1010),
            (XRegisters::s3, 0b1111),
            (XRegisters::s4, 0b1111),
        ],
    );
});
define_test!(and, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 0b0101),
            (XRegisters::s2, 0b1010),
            (XRegisters::s3, 0b0000),
            (XRegisters::s4, 0b0000),
        ],
    );
});
define_test!(sb_lb, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s2, 0x0101_0101_0101_0101),
            (XRegisters::s3, 0x01),
        ],
    );
});

define_test!(sh_lh, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s2, 0x0101_0101_0101_0101),
            (XRegisters::s3, 0x0101),
        ],
    );
});
define_test!(sw_lw, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s2, 0x0101_0101_0101_0101),
            (XRegisters::s3, 0x0101_0101),
        ],
    );
});
define_test!(sd_ld, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s2, 0x0101_0101_0101_0101),
            (XRegisters::s3, 0x0101_0101_0101_0101),
        ],
    );
});
define_test!(beq, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 366),
            (XRegisters::s2, 366),
            (XRegisters::s3, 1),
        ],
    );
});
define_test!(bne, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, -366_i64 as u64),
            (XRegisters::s2, 366),
            (XRegisters::s3, 1),
        ],
    );
});
define_test!(blt, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, -366_i64 as u64),
            (XRegisters::s2, 366),
            (XRegisters::s3, 1),
        ],
    );
});
define_test!(bltu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, -366_i64 as u64),
            (XRegisters::s2, 366),
            (XRegisters::s3, 0),
        ],
    );
});
define_test!(bge, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 366),
            (XRegisters::s2, -366_i64 as u64),
            (XRegisters::s3, 1),
        ],
    );
});
define_test!(bgeu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 366),
            (XRegisters::s2, -366_i64 as u64),
            (XRegisters::s3, 0),
        ],
    );
});
