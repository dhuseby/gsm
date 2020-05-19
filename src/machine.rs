use crate::{
    AppIO,
    Instruction,
    Script,
    Stack
};
use std::convert::From;

pub struct Machine<I: Clone>
{
    d: Stack<I>,
    r: Stack<usize>,
    s: Script<I>
}

impl<I: Clone + Instruction<I>> Machine<I>
{
    pub fn push(&mut self, i: I) {
        self.d.push(i);
    }

    pub fn pop(&mut self) -> Option<I> {
        self.d.pop()
    }

    pub fn pushr(&mut self, i: usize) {
        self.r.push(i);
    }

    pub fn popr(&mut self) -> Option<usize> {
        self.r.pop()
    }

    pub fn geti(&self, i: usize) -> Option<I> {
        self.s.get(i)
    }

    pub fn reset(&mut self) {
        self.d = Stack::<I>::new();
        self.r = Stack::<usize>::new();
        self.pushr(0);
    }

    pub fn execute(&mut self, io: &dyn AppIO<I>) -> Option<Stack<I>>
    {
        loop {
            if let Some(ip) = self.popr() {
                if let Some(instr) = self.geti(ip) {
                    instr.execute(ip, self, io);
                } else {
                    // end of script
                    return Some(self.d.clone());
                }
            } else {
                return None;
            }
        }
    }
}

impl<I: Clone> From<Script<I>> for Machine<I> {
    fn from(s: Script<I>) -> Self {
        Machine {
            d: Stack::<I>::new(),
            r: Stack::from(vec![0]),
            s: s
        }
    }
}
