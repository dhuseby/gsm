extern crate ccl;

use ccl::{
    Instruction,
    Machine,
    Script,
    Stack
};

#[derive(Clone, Copy)]
enum Instr {
    Add,
    Num(isize)
}

impl Instr {

    fn val(&self) -> Option<isize> {
        match *self {
            Instr::Add => None,
            Instr::Num(n) => Some(n)
        }
    }
}

impl Instruction<Instr> for Instr {

    fn name(&self) -> String {
        match self {
            Instr::Add => String::from("+"),
            Instr::Num(val) => val.to_string()
        }
    }

    fn arity(&self) -> usize {
        match self {
            Instr::Add => 2,
            Instr::Num(_) => 0
        }
    }

    fn execute(&self, stack: &mut Stack<Instr>) {
        match self {
            Instr::Add => {
                let a = stack.pop().unwrap().val().unwrap();
                let b = stack.pop().unwrap().val().unwrap();
                stack.push(Instr::Num(a + b))
            },
            Instr::Num(_) => {
                stack.push(*self)
            }
        }
    }
}

#[test]
fn simple_add() {
    let script = Script::from(vec![Instr::Num(3), Instr::Num(5), Instr::Add]);
    let mut machine = Machine::<Instr>::new();
   
    // execute the script
    let mut result = machine.execute(&mut script.clone()).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Instr::Num with value 8
    let r = result.pop().unwrap().val().unwrap();
    assert_eq!(r, 8);
}
