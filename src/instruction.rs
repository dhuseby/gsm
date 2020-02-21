use crate::Machine;

pub trait Instruction<T> {
    fn execute(&self, m: &mut Machine<T>) -> Option<usize>;
}
