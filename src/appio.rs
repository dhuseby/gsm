use crate::Machine;
use serde::{
    de,
    Deserialize,
    Deserializer
};
use std::{
    fmt::{
        self,
        Display,
        Formatter
    },
    io,
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Whence {
    Start,
    Cur,
    End
}

pub struct WhenceVisitor;

impl<'de> de::Visitor<'de> for WhenceVisitor {
    type Value = Whence;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Whence token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "START" => Ok(Whence::Start),
            "CUR" => Ok(Whence::Cur),
            "END" => Ok(Whence::End),
            &_ => Err(E::custom(format!("failed to parse '{}' as Whence", v)))
        }
    }
}

impl<'de> Deserialize<'de> for Whence {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Whence, D::Error> {
        d.deserialize_any(WhenceVisitor)
    }
}

impl Display for Whence {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Whence::Start => write!(f, "START"),
            Whence::Cur => write!(f, "CUR"),
            Whence::End => write!(f, "END")
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Mode {
    s: String,
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub plus: bool,
    pub binary: bool
}

impl FromStr for Mode {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Mode::default();
        match String::from_str(s) {
            Ok(s) => m.s = s,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad mode string"))
        }
        for c in s.chars() {
            match c {
                'r' => m.read = true,
                'w' => m.write = true,
                'a' => m.append = true,
                'b' => m.binary = true,
                '+' => {
                    m.read = true;
                    m.write = true;
                    m.plus = true;
                },
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown file mode flag"))
            }
        }
        Ok(m)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode {
            s: String::default(),
            read: false,
            write: false,
            append: false,
            plus: false,
            binary: false
        }
    }
}

pub struct ModeVisitor;

impl<'de> de::Visitor<'de> for ModeVisitor {
    type Value = Mode;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mode token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match Mode::from_str(v) {
            Ok(m) => Ok(m),
            Err(_) => Err(E::custom("failed to parse file mode"))
        }
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Mode, D::Error> {
        d.deserialize_any(ModeVisitor)
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.read {
            write!(f, "r")?;
        }
        if self.write {
            write!(f, "w")?;
        }
        if self.append {
            write!(f, "a")?;
        }
        if self.binary {
            write!(f, "b")?;
        }
        if self.plus {
            write!(f, "+")?;
        }
        Ok(())
    }
}

pub trait AppIO<I: Clone> {
    fn open(&self, m: &mut Machine<I>) -> io::Result<()>;
    fn read(&self, m: &mut Machine<I>) -> io::Result<()>;
    fn write(&self, m: &mut Machine<I>) -> io::Result<()>;
    fn seek(&self, m: &mut Machine<I>) -> io::Result<()>;
    fn close(&self, m: &mut Machine<I>) -> io::Result<()>;
}

