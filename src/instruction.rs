use crate::Machine;

pub trait Instruction<T> {
    fn name(&self) -> String;
    fn execute(&self, m: &mut Machine<T>) -> Option<usize>;
}
