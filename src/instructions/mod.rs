mod amo;
mod branch;
mod instruction;
mod load;
mod op;
mod op_imm;
mod op_immw;
mod opw;
mod store;
mod system;
mod types;

pub use self::instruction::decode_and_execute;
