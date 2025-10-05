use std::{fs, path::PathBuf};

pub fn load_binary(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets/result")
        .join(format!("{name}.bin"));

    fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {path:?}: {e}"))
}

#[macro_export]
macro_rules! define_test {
    ($fn_name:ident, |$arg:ident| $body:block) => {
        #[test]
        fn $fn_name() {
            let mut cpu = Cpu::new();

            let bin = crate::helper::load_binary(stringify!($fn_name));
            cpu.mmu.inject(DRAM_BASE, &bin);
            cpu.run();

            let $arg = cpu;
            $body;
        }
    };
}
