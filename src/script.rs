use serde::{
    de::{
        self,
        IntoDeserializer
    },
    Deserialize,
    Deserializer,
    Serialize,
    Serializer
};
use std::{
    clone::Clone,
    convert::From,
    fmt,
    marker::PhantomData,
    vec::Vec
};

#[derive(Clone, Debug, PartialEq)]
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
        for (n, i) in self.0.iter().enumerate() {
            if n > 0 {
                write!(f, " ").unwrap();
            }
            write!(f, "{}", i).unwrap();
        }
        Ok(())
    }
}

impl<I: Clone + fmt::Display> Serialize for Script<I> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>
    {
        s.serialize_str(format!("{}", self).as_str())
    }
}

struct ScriptVisitor<I>(PhantomData<fn() -> I>);

impl<'de, I: Clone + Deserialize<'de> + fmt::Debug> de::Visitor<'de> for ScriptVisitor<I> {
    type Value = Script<I>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Script string")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {

        let mut v: Vec<I> = Vec::new();
        for t in s.split_whitespace() {
            let i: I = Deserialize::deserialize(t.into_deserializer())?;
            v.push(i);
        }
        Ok(Script::from(v))
    }
}

impl<'de, I: Clone + Deserialize<'de> + fmt::Debug> Deserialize<'de> for Script<I> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Script<I>, D::Error>
    {
        d.deserialize_str(ScriptVisitor(PhantomData))
    }
}

