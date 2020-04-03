/*
use serde::{
    de::Visitor,
    Deserialize,
    Deserializer,
    ser::SerializeSeq,
    Serialize,
    Serializer
};
*/

use std::{
    clone::Clone,
    convert::From,
    fmt,
    //result::Result,
    vec::Vec
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Script<I: Clone>(Vec<I>);

impl<I: Clone> Script<I> {
    pub fn new() -> Self {
        Script(vec![])
    }

    pub fn get(&self, l: usize) -> Option<I> {
        if let Some(i) = self.0.get(l) {
            return Some(i.clone());
        }
        None
    }
}

impl<I: Clone> From<Vec<I>> for Script<I> {
    fn from(s: Vec<I>) -> Self {
        Script(s)
    }
}

impl<I: Clone + fmt::Display> fmt::Display for Script<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |r, i| {
            r.and_then(|_| writeln!(f, "{}", i))
        })
    }
}

/*
impl<I: Clone + Serialize> Serialize for Script<I> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>
    {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for i in self.0 {
            seq.serialize_element(&i)?;
        }
        seq.end()
    }
}

impl<'de, I: Clone, V: Visitor<'de>> Deserialize<'de> for Script<I> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Script<I>, D::Error>
    {
        d.deserialize_any(V)
    }
}
*/
