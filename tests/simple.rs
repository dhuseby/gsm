extern crate gsm;
use gsm::{
    Instruction,
    Machine,
    Script
};
use serde::{
    de,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer
};
use std::{
    fmt
};

#[derive(Clone, Copy)]
enum Instr {
    Add,
    Num(isize),
    Boolean(bool),
    If,
    Else,
    Fi
}

impl Serialize for Instr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>
    {
        match *self {
            Instr::Add => s.serialize_str("+"),
            Instr::Num(i) => s.serialize_i64(i as i64),
            Instr::Boolean(b) => s.serialize_bool(b),
            Instr::If => s.serialize_str("IF"),
            Instr::Else => s.serialize_str("ELSE"),
            Instr::Fi => s.serialize_str("FI")
        }
    }
}

struct InstrVisitor;

impl<'de> de::Visitor<'de> for InstrVisitor {
    type Value = Instr;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instr token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        println!("visit_str({})", v);
        match v {
            "+" => Ok(Instr::Add),
            "IF" => Ok(Instr::If),
            "ELSE" => Ok(Instr::Else),
            "FI" => Ok(Instr::Fi),
            &_ => Err(E::custom(format!("unknown token: {}", v)))
        }
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        println!("visit_bool({})", v);
        Ok(Instr::Boolean(v))
    }

    fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
        println!("visit_i8({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
        println!("visit_i16({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
        println!("visit_i32({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        println!("visit_i64({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<Self::Value, E> {
        println!("visit_i128({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
        println!("visit_u8({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
        println!("visit_u16({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
        println!("visit_u32({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        println!("visit_u64({})", v);
        Ok(Instr::Num(v as isize))
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<Self::Value, E> {
        println!("visit_u128({})", v);
        Ok(Instr::Num(v as isize))
    }
}

impl<'de> Deserialize<'de> for Instr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Instr, D::Error>
    {
        d.deserialize_any(InstrVisitor)
    }
}

struct IfMatch {
    ifi: usize,
    elsei: Option<usize>,
    fii: usize
}

fn find_matching_elsefi(m: &Machine<Instr>, i: usize) -> Option<IfMatch> {
    let mut ret = IfMatch { ifi: i, elsei: None, fii: 0 };
    let mut ip = ret.ifi + 1;
    loop {
        match m.geti(ip) {
            // skip over instructions that aren't if/else/fi
            Some(Instr::Add) |
            Some(Instr::Num(_)) |
            Some(Instr::Boolean(_)) => {
                ip += 1;
            },

            Some(Instr::If) => {
                // this is an inner 'IF' that we need to find the end of.
                let im = match find_matching_elsefi(m, ip) {
                    Some(inner) => inner,
                    None => return None
                };

                // skip to the index just after the closing 'FI'
                ip = im.fii + 1;
            },

            Some(Instr::Else) => {
                // this is an 'ELSE' to our starting 'IF' so we just record the
                // index in the result and move on.
                ret.elsei = Some(ip);
                ip += 1;
            },

            Some(Instr::Fi) => {
                // we found our matching 'FI' so return the result
                ret.fii = ip;
                return Some(ret);
            },

            // if we don't get an instruction we've reached the end of the
            // script without finding the matching 'FI' so return None to
            // signal the error.
            None => {
                return None;
            }
        }
    }
}

impl Instruction<Instr> for Instr {

    fn execute(&self, ip: usize, m: &mut Machine<Instr>) {
        match self {
            Instr::Add => {
                if let Some(Instr::Num(r)) = m.pop() {
                    if let Some(Instr::Num(l)) = m.pop() {
                        m.push(Instr::Num(l + r));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            Instr::If => {
                // find the location of the matching 'ELSE' if any and 'FI'
                if let Some(ifm) = find_matching_elsefi(m, ip) {
                    // get the Boolean from the stack
                    if let Some(Instr::Boolean(b)) = m.pop() {
                        if b {
                            // the boolean is true so continue with the code that is
                            // between this if and it's matching 'ELSE'

                            // first record where we need to go after this block
                            m.pushr(ifm.fii + 1);

                            // then tell the machine the correct next instruction
                            m.pushr(ip + 1);
                            return;
                        } else {
                            // the boolean is false so skip to the instruction after
                            // the 'ELSE' if there is one, otherwise skip to after the
                            // 'FI'
                            let next_ip = match ifm.elsei {
                                Some(i) => {
                                    // we're executing the 'ELSE' block so we need to
                                    // push a frame with the correct next instruction
                                    m.pushr(ifm.fii + 1);

                                    // set the next instruction pointer to the
                                    // instruction after the 'ELSE'
                                    i + 1
                                },

                                // No 'ELSE' clause so just skip to the instruction
                                // after the 'FI'. There is no need to record a frame.
                                None => ifm.fii + 1
                            };

                            m.pushr(next_ip);
                            return;
                        }
                    }
                }
                panic!();
            },
            Instr::Else => {
                // we see an 'ELSE' so this can only be because we previously
                // encoutered in 'IF' and the boolean was true and the
                // if/else/fi block had an else. the right thing to do here is
                // to pop the frame from the machine and skip to the next
                // instruction pointer.
                if let Some(next_ip) = m.popr() {
                    m.pushr(next_ip);
                    return;
                }
                panic!();
            }
            Instr::Fi => {
                // we finished executing an 'IF' or 'ELSE' block so pop the
                // frame and continue
                if let Some(next_ip) = m.popr() {
                    m.pushr(next_ip);
                    return;
                }
                panic!();
            },
            Instr::Num(_) |
            Instr::Boolean(_) => {
                // push the value onto the stack and keep going
                m.push(*self);
                m.pushr(ip + 1);
                return;
            }
        }
    }
}

impl fmt::Display for Instr {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Add => writeln!(f, "+"),
            Instr::If => writeln!(f, "IF"),
            Instr::Else => writeln!(f, "ELSE"),
            Instr::Fi => writeln!(f, "FI"),
            Instr::Num(val) => writeln!(f, "{}", val),
            Instr::Boolean(b) => {
                let val = if *b { "true" } else { "false" };
                writeln!(f, "{}", val)
            }
        }
    }
}

#[test]
fn simple_add() {

    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Num(3),
        Instr::Num(5),
        Instr::Add
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Instr::Num with value 8
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 8),
        _ => panic!()
    }
}

#[test]
fn simple_branching_0() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(true),
        Instr::If,
            Instr::Num(1),
        Instr::Else,
            Instr::Num(2),
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 1
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 1),
        _ => panic!()
    }
}

#[test]
fn simple_branching_1() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(false),
        Instr::If,
            Instr::Num(1),
        Instr::Else,
            Instr::Num(2),
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 2
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 2),
        _ => panic!()
    }
}

#[test]
fn nested_branching_0() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(true),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(true),
            Instr::If,
                Instr::Num(3),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 4
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}

#[test]
fn nested_branching_1() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(true),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 5
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 5),
        _ => panic!()
    }
}

#[test]
fn nested_branching_2() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(false),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
            Instr::Boolean(true),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 5
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 5),
        _ => panic!()
    }
}

#[test]
fn nested_branching_3() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(false),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 6
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 6),
        _ => panic!()
    }
}

#[test]
fn serialization_json() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(false),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Fi
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#"[false,"IF",1,false,"IF",3,"ELSE",4,"FI","+","ELSE",2,false,"IF",3,"ELSE",4,"FI","+","FI"]"#);
}

#[test]
fn deserialization_json() {
    let s = r#"[false,"IF",1,false,"IF",3,"ELSE",4,"FI","+","ELSE",2,false,"IF",3,"ELSE",4,"FI","+","FI"]"#;
    let script: Script<Instr> = serde_json::from_str(s).unwrap();
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 6
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 6),
        _ => panic!()
    }
}

#[test]
fn serialization_cbor() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Instr::Boolean(false),
        Instr::If,
            Instr::Num(1),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Else,
            Instr::Num(2),
            Instr::Boolean(false),
            Instr::If,
                Instr::Num(3),
            Instr::Else,
                Instr::Num(4),
            Instr::Fi,
            Instr::Add,
        Instr::Fi
    ]);
    let s = serde_cbor::to_vec(&script).unwrap();
    let c: Vec<u8> = vec![0x94, 0xf4, 0x62, 0x49, 0x46, 0x01, 0xf4, 0x62, 0x49,
                          0x46, 0x03, 0x64, 0x45, 0x4c, 0x53, 0x45, 0x04, 0x62,
                          0x46, 0x49, 0x61, 0x2b, 0x64, 0x45, 0x4c, 0x53, 0x45,
                          0x02, 0xf4, 0x62, 0x49, 0x46, 0x03, 0x64, 0x45, 0x4c,
                          0x53, 0x45, 0x04, 0x62, 0x46, 0x49, 0x61, 0x2b, 0x62,
                          0x46, 0x49];
    assert_eq!(s, c);
}

#[test]
fn deserialization_cbor() {
    let c: Vec<u8> = vec![0x94, 0xf4, 0x62, 0x49, 0x46, 0x01, 0xf4, 0x62, 0x49,
                          0x46, 0x03, 0x64, 0x45, 0x4c, 0x53, 0x45, 0x04, 0x62,
                          0x46, 0x49, 0x61, 0x2b, 0x64, 0x45, 0x4c, 0x53, 0x45,
                          0x02, 0xf4, 0x62, 0x49, 0x46, 0x03, 0x64, 0x45, 0x4c,
                          0x53, 0x45, 0x04, 0x62, 0x46, 0x49, 0x61, 0x2b, 0x62,
                          0x46, 0x49];
    let script: Script<Instr> = serde_cbor::from_reader(c.as_slice()).unwrap();
    let mut machine = Machine::from(script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 6
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 6),
        _ => panic!()
    }
}
