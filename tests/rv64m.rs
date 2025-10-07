mod helper;
use crate::helper::assert_xregs;
use risc_v::{
    components::{registers::XRegisters, system_bus::DRAM_BASE},
    cpu::Cpu,
};

/* @Note for op instructions:
 * s1-s2: dword input
 * s3: dword output
 * s4: word output
 * */

define_test!(mul, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 2),
            (XRegisters::s2, 2),
            (XRegisters::s3, 4),
            (XRegisters::s4, 4),
        ],
    );
});
define_test!(mulh, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 3),
            (XRegisters::s3, u64::MAX),
        ],
    );
});
define_test!(mulhsu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 3),
            (XRegisters::s3, u64::MAX),
        ],
    );
});
define_test!(mulhu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 3),
            (XRegisters::s3, 2),
        ],
    );
});
define_test!(div, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 3),
            (XRegisters::s3, 0),
            (XRegisters::s4, 0),
        ],
    );
});
define_test!(divu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 3),
            (XRegisters::s3, u64::MAX / 3),
            (XRegisters::s4, (u32::MAX / 3) as u64),
        ],
    );
});
define_test!(div_bz, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 1),
            (XRegisters::s2, 0),
            (XRegisters::s3, u64::MAX),
            (XRegisters::s4, u32::MAX as u64),
        ],
    );
});
define_test!(rem, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 4),
            (XRegisters::s3, u64::MAX),
            (XRegisters::s4, u32::MAX as u64),
        ],
    );
});
define_test!(remu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, u64::MAX),
            (XRegisters::s2, 4),
            (XRegisters::s3, 3),
            (XRegisters::s4, 3),
        ],
    );
});
define_test!(rem_bz, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::s1, 1),
            (XRegisters::s2, 0),
            (XRegisters::s3, 1),
        ],
    );
});
