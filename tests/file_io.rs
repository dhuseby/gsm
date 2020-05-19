extern crate gsm;
use bytes::{
    BufMut,
    Bytes,
    BytesMut
};
use gsm::{
    AppIO,
    Instruction,
    Machine,
    Script
};
use hex;
use serde::{
    de,
    Deserialize,
    Deserializer
};
use std::{
    fmt,
    fs::{
        self,
        File,
        OpenOptions
    },
    io::{
        self,
        BufRead,
        BufReader,
        Seek,
        SeekFrom,
        Write
    },
    path::PathBuf,
    rc::Rc,
    str::FromStr
};

#[derive(Clone, Debug)]
enum Instr {
    // can be in a script
    Open,
    Read,
    Write,
    Seek,
    Close,
    Num(isize),
    Binary(Bytes),
    Text(String),

    // IO flags from GSM that can be in a script
    Whence(gsm::Whence),
    Mode(gsm::Mode),

    // holder that can be on the stack but not in a script
    IOHandle{ f: Rc<File>, binary: bool }
}

struct InstrVisitor;

impl<'de> de::Visitor<'de> for InstrVisitor {
    type Value = Instr;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instr token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let mv = gsm::ModeVisitor;
        let wv = gsm::WhenceVisitor;

        if let Ok(m) = mv.visit_str::<E>(v) {
            return Ok(Instr::Mode(m));
        } else if let Ok(w) = wv.visit_str::<E>(v) {
            return Ok(Instr::Whence(w));
        } else {
            match v {
                "OPEN" => return Ok(Instr::Open),
                "READ" => return Ok(Instr::Read),
                "WRITE" => return Ok(Instr::Write),
                "SEEK" => return Ok(Instr::Seek),
                "CLOSE" => return Ok(Instr::Close),
                &_ => {
                    if let Ok(i) = v.parse::<isize>() {
                        return Ok(Instr::Num(i));
                    } else if let Ok(h) = hex::decode(v) {
                        return Ok(Instr::Binary(Bytes::copy_from_slice(h.as_slice())));
                    } else {
                        return Ok(Instr::Text(v.to_string()));
                    }
                }
            }
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
            Instr::Open => write!(f, "OPEN"),
            Instr::Read => write!(f, "READ"),
            Instr::Write => write!(f, "WRITE"),
            Instr::Seek => write!(f, "SEEK"),
            Instr::Close => write!(f, "CLOSE"),
            Instr::Num(val) => write!(f, "{}", val),
            Instr::Binary(b) => write!(f, "{}", hex::encode(b.as_ref())),
            Instr::Text(s) => write!(f, "{}", s),
            Instr::Whence(w) => write!(f, "{}", w),
            Instr::Mode(m) => write!(f, "{}", m),
            Instr::IOHandle{ f:_, binary:_ } => panic!("cannot serialie IOHandle")
        }
    }
}

struct FileIO;

impl AppIO<Instr> for FileIO {
    fn open(&self, m: &mut Machine<Instr>) -> io::Result<()> {
        if let Some(Instr::Mode(mode)) = m.pop() {
            if let Some(Instr::Text(p)) = m.pop() {
                let path = PathBuf::from(p);
                let create = mode.write || mode.append;
                let truncate = mode.write;
                let oo  = OpenOptions::new()
                                      .read(mode.read || mode.plus)
                                      .write(mode.write || mode.plus)
                                      .append(mode.append)
                                      .create(create)
                                      .truncate(truncate)
                                      .open(path.as_path());

                let f = match oo {
                    Ok(f) => f,
                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            format!("failed to open file '{}'",
                                    path.to_str().unwrap())))
                };

                m.push(Instr::IOHandle{ f: Rc::new(f), binary: mode.binary });
                return Ok(());
            }
            return Err(io::Error::new(io::ErrorKind::InvalidData, "no file path"));
        }
        return Err(io::Error::new(io::ErrorKind::InvalidData, "no file mode"));
    }

    fn read(&self, m: &mut Machine<Instr>) -> io::Result<()> {
        if let Some(Instr::Num(i)) = m.pop() {
            if let Some(Instr::IOHandle{ mut f, binary}) = m.pop() {
                let fh = Rc::get_mut(&mut f).unwrap();
                let mut br = BufReader::with_capacity(i as usize, fh);
                let buf = br.fill_buf()?;
                if binary {
                    m.push(Instr::Binary(Bytes::copy_from_slice(buf)));
                } else {
                    m.push(Instr::Text(String::from_utf8_lossy(buf).to_string()));
                }
                m.push(Instr::IOHandle{ f, binary });
            }
        }
        Ok(())
    }

    fn write(&self, m: &mut Machine<Instr>) -> io::Result<()> {
        match m.pop() {
            Some(Instr::Binary(b)) => {
                if let Some(Instr::IOHandle{ mut f, binary }) = m.pop() {
                    if !binary {
                        return Err(io::Error::new(io::ErrorKind::Other, "writing binary to a text file"));
                    }
                    let fh = Rc::get_mut(&mut f).unwrap();
                    fh.write(b.as_ref())?;
                    m.push(Instr::IOHandle{ f, binary });
                }
                return Ok(())
            },
            Some(Instr::Text(s)) => {
                if let Some(Instr::IOHandle{ mut f, binary }) = m.pop() {
                    if binary {
                        return Err(io::Error::new(io::ErrorKind::Other, "writing text to a binary file"));
                    }
                    let fh = Rc::get_mut(&mut f).unwrap();
                    fh.write(s.as_ref())?;
                    m.push(Instr::IOHandle{ f, binary });
                }
                return Ok(())
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "no data to write"))
        }
    }

    fn seek(&self, m: &mut Machine<Instr>) -> io::Result<()> {
        if let Some(Instr::Whence(w)) = m.pop() {
            if let Some(Instr::Num(i)) = m.pop() {
                if let Some(Instr::IOHandle{ mut f, binary }) = m.pop() {
                    let fh = Rc::get_mut(&mut f).unwrap();
                    match w {
                        gsm::Whence::Start => fh.seek(SeekFrom::Start(i as u64))?,
                        gsm::Whence::End => fh.seek(SeekFrom::End(i as i64))?,
                        gsm::Whence::Cur => fh.seek(SeekFrom::Current(i as i64))?
                    };
                    m.push(Instr::IOHandle{ f, binary });
                }
            }
        }
        Ok(())
    }

    fn close(&self, m: &mut Machine<Instr>) -> io::Result<()> {
        if let Some(Instr::IOHandle{ f, binary:_ }) = m.pop() {
            drop(f);
        }
        Ok(())
    }
}

impl Instruction<Instr> for Instr {
    fn execute(&self, ip: usize, m: &mut Machine<Instr>, io: &dyn AppIO<Instr>) {
        match self {
            Instr::Num(_) |
            Instr::Binary(_) |
            Instr::Text(_) |
            Instr::Whence(_) |
            Instr::Mode(_) => {
                m.push(self.clone());
                m.pushr(ip + 1);
                return;
            },
            Instr::Open => {
                if let Err(e) = io.open(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            Instr::Read => {
                if let Err(e) = io.read(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            Instr::Write => {
                if let Err(e) = io.write(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            Instr::Seek => {
                if let Err(e) = io.seek(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            Instr::Close => {
                if let Err(e) = io.close(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            Instr::IOHandle{ f:_, binary:_ } => panic!()
        }
    }
}


#[test]
fn open_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text("LICENSE".to_string()),
        Instr::Mode(gsm::Mode::from_str("r").unwrap()),
        Instr::Open
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Instr::IOHandle
    match result.pop() {
        Some(Instr::IOHandle{ f:_, binary:_ }) => {},
        _ => panic!()
    }
}

#[test]
fn open_close_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text("LICENSE".to_string()),
        Instr::Mode(gsm::Mode::from_str("r").unwrap()),
        Instr::Open,
        Instr::Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there shouldn't be anything on the stack
    assert_eq!(result.size(), 0 as usize);
}

#[test]
fn read_text_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text("LICENSE".to_string()),
        Instr::Mode(gsm::Mode::from_str("r").unwrap()),
        Instr::Open,
        Instr::Num(128),
        Instr::Read,
        Instr::Close
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Instr::Text of length 128
    match result.pop() {
        Some(Instr::Text(s)) => {
            assert_eq!(s.len(), 128);
        },
        _ => panic!()
    }
}

#[test]
fn read_binary_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text("LICENSE".to_string()),
        Instr::Mode(gsm::Mode::from_str("rb").unwrap()),
        Instr::Open,
        Instr::Num(128),
        Instr::Read,
        Instr::Close
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Instr::Binary of length 128
    match result.pop() {
        Some(Instr::Binary(b)) => {
            assert_eq!(b.len(), 128);
        },
        _ => panic!()
    }
}

#[test]
fn write_text_file() {
    let fname = "test.txt";
    let data = "When in the Course of human events...".to_string();
    let len = data.len() as u64;

    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text(fname.to_string()),
        Instr::Mode(gsm::Mode::from_str("w").unwrap()),
        Instr::Open,
        Instr::Text(data),
        Instr::Write,
        Instr::Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), len);
    fs::remove_file(fname).unwrap();
}

#[test]
fn write_binary_file() {
    let fname = "test.bin";
    let mut b = BytesMut::new();
    let data = hex::decode("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169").unwrap();
    let len = data.len() as u64;
    b.put(data.as_ref());


    // construct the script and load it into the machine
    let script = Script::from(vec![
        Instr::Text(fname.to_string()),
        Instr::Mode(gsm::Mode::from_str("wb").unwrap()),
        Instr::Open,
        Instr::Binary(b.freeze()),
        Instr::Write,
        Instr::Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), len);
    fs::remove_file(fname).unwrap();
}

#[test]
fn deserialization_json() {
    let s = r#""script.txt w OPEN blah WRITE CLOSE""#;
    let script: Script<Instr> = serde_json::from_str(s).unwrap();
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // the stack should be empty
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata("script.txt").unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), 4);
    fs::remove_file("script.txt").unwrap();
}


