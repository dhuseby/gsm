use std::clone::Clone;
use std::vec::Vec;

#[derive(Clone)]
pub struct Script<T>(Vec<T>);

impl<T: Clone> Script<T> {

    pub fn new() -> Self {
        Script(vec![])
    }

    pub fn clone(&self) -> Self {
        Script(self.0.clone())
    }

    pub fn push_back(&mut self, i: T) {
        self.0.push(i);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.0.is_empty() {
            return None;
        }
        
        Some(self.0.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Clone> From<Vec<T>> for Script<T> {

    fn from(s: Vec<T>) -> Self {
        Script(s.clone())
    }
}
