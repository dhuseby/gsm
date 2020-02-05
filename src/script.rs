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

    pub fn get(&self, idx: usize) -> Option<T> {
        match self.0.get(idx) {
            Some(i) => Some(i.clone()),
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone> From<Vec<T>> for Script<T> {

    fn from(s: Vec<T>) -> Self {
        Script(s.clone())
    }
}
