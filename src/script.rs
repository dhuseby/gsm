use std::clone::Clone;
use std::fmt;
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

impl<T: Clone + fmt::Display> fmt::Display for Script<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |r, i| {
            r.and_then(|_| writeln!(f, "{}", i))
        })
    }
}
