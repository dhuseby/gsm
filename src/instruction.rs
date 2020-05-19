use crate::{
    AppIO,
    Machine
};
use std::clone::Clone;

pub trait Instruction<I: Clone> {
    fn execute(&self, ip: usize, m: &mut Machine<I>, io: &dyn AppIO<I>);
}
