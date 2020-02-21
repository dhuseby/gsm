use std::fmt;
use std::vec::Vec;

#[derive(Clone)]
pub struct Stack<T>(Vec<T>);

impl<T: Clone> Stack<T> {

    pub fn new() -> Self {
        Stack(vec![])
    }

    pub fn clone(&self) -> Self {
        Stack(self.0.clone())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn push(&mut self, t: T) {
        self.0.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Stack<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = self.0.clone();
        s.reverse();
        s.iter().fold(Ok(()), |r, i| {
            r.and_then(|_| writeln!(f, "{}", i))
        })
    }
}
