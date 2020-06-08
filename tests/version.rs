extern crate gsm;
use gsm::{
    AppIO,
    Instruction,
    Machine,
    MachineBuilder,
    Script
};
use semver::{
    Version,
    VersionReq
};
use serde::{
    de,
    Deserialize,
    Deserializer
};
use std::{
    fmt,
    io
};

#[derive(Clone, Debug, PartialEq)]
enum Instr {
    Text(String),
    Boolean(bool),
    Version
}

struct InstrVisitor;

impl<'de> de::Visitor<'de> for InstrVisitor {
    type Value = Instr;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instr token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "VERSION" => Ok(Instr::Version),
            "true" => Ok(Instr::Boolean(true)),
            "false" => Ok(Instr::Boolean(false)),
            &_ => Ok(Instr::Text(v.to_string()))
        }
    }
}

impl<'de> Deserialize<'de> for Instr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Instr, D::Error> {
        d.deserialize_any(InstrVisitor)
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Version => write!(f, "VERSION"),
            Instr::Text(s) => write!(f, "{}", &s),
            Instr::Boolean(b) => write!(f, "{}", b)
        }
    }
}

struct NullIO;

impl AppIO<Instr> for NullIO {
    fn open(&self, _m: &mut Machine<Instr>) -> io::Result<()> { Ok(()) }
    fn read(&self, _m: &mut Machine<Instr>) -> io::Result<()> { Ok(()) }
    fn write(&self, _m: &mut Machine<Instr>) -> io::Result<()> { Ok(()) }
    fn seek(&self, _m: &mut Machine<Instr>) -> io::Result<()> { Ok(()) }
    fn close(&self, _m: &mut Machine<Instr>) -> io::Result<()> { Ok(()) }
}

impl Instruction<Instr> for Instr {

    fn execute(&self, ip: usize, m: &mut Machine<Instr>, _io: &dyn AppIO<Instr>) {
        match self {
            Instr::Version => {
                if let Some(Instr::Text(s)) = m.pop() {
                    if let Ok(v) = Version::parse(&s) {
                        m.push(Instr::Boolean(m.version_check(&v)));
                        m.pushr(ip + 1);
                        return;
                    }
                }
            }
            Instr::Boolean(_) |
            Instr::Text(_) => {
                // push the value onto the stack and keep going
                m.push(self.clone());
                m.pushr(ip + 1);
                return;
            }
        }
        panic!();
    }
}


#[test]
fn version_0() {
    let script = Script::from(vec![
        Instr::Text("1.0.0".to_string()),
        Instr::Version
    ]);
    let mut machine = MachineBuilder::new()
        .script(&script)
        .version_req(&VersionReq::parse(">= 1.0.0").unwrap())
        .build();
    let mut result = machine.execute(&NullIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be a Boolean with value "true"
    match result.pop() {
        Some(Instr::Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
fn version_1() {
    let script = Script::from(vec![
        Instr::Text("0.1.0".to_string()),
        Instr::Version
    ]);
    let mut machine = MachineBuilder::new()
        .script(&script)
        .version_req(&VersionReq::parse(">= 1.0.0").unwrap())
        .build();
    let mut result = machine.execute(&NullIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be a Boolean with value "true"
    match result.pop() {
        Some(Instr::Boolean(b)) => assert_eq!(b, false),
        _ => panic!()
    }
}
