mod instruction;
mod load;
mod op;
mod op_imm;
mod store;
mod system;

pub use self::instruction::decode_and_execute;
