extern crate gsm;

use gsm::{
    Instruction,
    Machine,
    Script
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

struct IfMatch {
    ifi: usize,
    elsei: Option<usize>,
    fii: usize
}

fn find_matching_elsefi(m: &Machine<Instr>, i: usize) -> Option<IfMatch> {
    let mut ret = IfMatch { ifi: i, elsei: None, fii: 0 };
    let mut ip = ret.ifi + 1;
    loop {
        match m.get_instruction(ip) {
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

    fn name(&self) -> String {
        match self {
            Instr::Add => String::from("+"),
            Instr::If => String::from("IF"),
            Instr::Else => String::from("ELSE"),
            Instr::Fi => String::from("FI"),
            Instr::Num(val) => val.to_string(),
            Instr::Boolean(b) => {
                if *b {
                    String::from("TRUE")
                } else {
                    String::from("FALSE")
                }
            }
        }
    }

    fn execute(&self, m: &mut Machine<Instr>) -> Option<usize> {
        match self {
            Instr::Add => {
                if let Instr::Num(r) = m.pop() {
                    if let Instr::Num(l) = m.pop() {
                        m.push(Instr::Num(l + r));
                        return m.next_ip();
                    }
                }
                panic!();
            },
            Instr::If => {
                // find the location of the matching 'ELSE' if any and 'FI'
                let ifm = match find_matching_elsefi(m, m.get_ip()) {
                    Some(ifefi) => ifefi,
                    None => return None
                };

                // get the Boolean from the stack
                if let Instr::Boolean(b) = m.pop() {
                    if b {
                        // the boolean is true so continue with the code that is
                        // between this if and it's matching 'ELSE'
                        
                        // first record where we need to go after this block
                        m.push_frame(ifm.fii + 1);

                        // then tell the machine the correct next instruction
                        return m.next_ip();
                    } else {
                        // the boolean is false so skip to the instruction after
                        // the 'ELSE' if there is one, otherwise skip to after the
                        // 'FI'
                        let next_ip = match ifm.elsei {
                            Some(i) => {
                                // we're executing the 'ELSE' block so we need to
                                // push a frame with the correct next instruction
                                m.push_frame(ifm.fii + 1);

                                // set the next instruction pointer to the
                                // instruction after the 'ELSE'
                                i + 1
                            },

                            // No 'ELSE' clause so just skip to the instruction
                            // after the 'FI'. There is no need to record a frame.
                            None => ifm.fii + 1
                        };

                        return Some(next_ip);
                    }
                }
                None
            },
            Instr::Else => {
                // we see an 'ELSE' so this can only be because we previously
                // encoutered in 'IF' and the boolean was true and the
                // if/else/fi block had an else. the right thing to do here is
                // to pop the frame from the machine and skip to the next
                // instruction pointer.
                let next_ip = match m.pop_frame() {
                    Some(i) => i,
                    None => return None
                };

                return Some(next_ip);
            }
            Instr::Fi => {
                // we finished executing an 'IF' or 'ELSE' block so pop the
                // frame and continue
                let next_ip = match m.pop_frame() {
                    Some(i) => i,
                    None => return None
                };

                return Some(next_ip);
            },
            Instr::Num(_) |
            Instr::Boolean(_) => {
                // push the value onto the stack and keep going
                m.push(*self);
                return m.next_ip();
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
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
    let mut machine = Machine::<Instr>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single Num value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Num should have the value of 6
    match result.pop() {
        Some(Instr::Num(num)) => assert_eq!(num, 6),
        _ => panic!()
    }
}

