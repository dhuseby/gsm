#[macro_use]
extern crate serde_derive;

pub mod instruction;
pub use crate::instruction::Instruction;

pub mod machine;
pub use crate::machine::Machine;

pub mod script;
pub use crate::script::Script;

pub mod stack;
pub use crate::stack::Stack;
