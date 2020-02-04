use crate::{
    Instruction,
    Script,
    Stack
};

pub struct Machine<T> {
    stack: Stack<T>
}

impl<T: Instruction<T> + Clone> Machine<T> {

    pub fn new() -> Self {
        Machine {
            stack: Stack::new()
        }
    }

    pub fn reboot(&mut self) {
        self.stack.clear()
    }

    pub fn execute(&mut self, s: &mut Script<T>) -> Option<Stack<T>> {
        loop {
            if s.is_empty() {
                return Some(self.stack.clone());
            }

            if let Some(instr) = s.pop_front() {
                if instr.arity() > self.stack.size() {
                    return None;
                }

                instr.execute(&mut self.stack);
            }
        }
    }
}

