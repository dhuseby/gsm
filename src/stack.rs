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

    pub fn push(&mut self, i: T) {
        self.0.push(i);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}
