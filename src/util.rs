#[macro_export]
macro_rules! sign_extend {
    ($value:expr, $sign_bit:expr) => {{
        let val = $value; //so that an expression can be passed
        let bits_size = std::mem::size_of_val(&val) * 8;
        let shift = bits_size - $sign_bit - 1;
        ((val << shift) >> shift)
    }};
}
