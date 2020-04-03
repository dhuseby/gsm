use std::{
    clone::Clone,
    convert::From,
    fmt,
    vec::Vec
};

#[derive(Clone)]
pub struct Stack<T: Clone>(Vec<T>);

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack(vec![])
    }

    pub fn push(&mut self, i: T) {
        self.0.push(i);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<I: Clone> From<Vec<I>> for Stack<I> {
    fn from(s: Vec<I>) -> Self {
        Stack(s)
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().rev().fold(Ok(()), |r, i| {
            r.and_then(|_| writeln!(f, "{}", i))
        })
    }
}
