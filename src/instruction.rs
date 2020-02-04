use crate::Stack;

pub trait Instruction<T> {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn execute(&self, stack: &mut Stack<T>);
}
