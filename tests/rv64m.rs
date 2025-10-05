mod helper;
use crate::helper::assert_xregs;
use risc_v::{
    components::{registers::XRegisters, system_bus::DRAM_BASE},
    cpu::Cpu,
};

define_test!(mul, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 2),
            (XRegisters::t4, 2),
            (XRegisters::t5, 4),
        ],
    );
});
define_test!(mulh, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 3),
            (XRegisters::t5, u64::MAX),
        ],
    );
});
define_test!(mulhsu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 3),
            (XRegisters::t5, u64::MAX),
        ],
    );
});
define_test!(mulhu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 3),
            (XRegisters::t5, 2),
        ],
    );
});
define_test!(div, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 3),
            (XRegisters::t5, 0),
        ],
    );
});
define_test!(divu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 3),
            (XRegisters::t5, u64::MAX / 3),
        ],
    );
});
define_test!(div_bz, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 1),
            (XRegisters::t4, 0),
            (XRegisters::t5, u64::MAX),
        ],
    );
});
define_test!(rem, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 4),
            (XRegisters::t5, u64::MAX),
        ],
    );
});
define_test!(remu, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, u64::MAX),
            (XRegisters::t4, 4),
            (XRegisters::t5, 3),
        ],
    );
});
define_test!(rem_bz, |cpu| {
    assert_xregs(
        &cpu,
        &[
            (XRegisters::t3, 1),
            (XRegisters::t4, 0),
            (XRegisters::t5, 1),
        ],
    );
});
