use crate::{
    Instruction,
    Script,
    Stack
};

pub struct Machine<T> {
    ip: usize,
    script: Script<T>,
    data_stack: Stack<T>,
    exe_stack: Vec<usize>
}

impl<T: Instruction<T> + Clone> Machine<T> {

    pub fn new() -> Self {
        Machine {
            ip: 0,
            script: Script::new(),
            data_stack: Stack::new(),
            exe_stack: vec![]
        }
    }

    pub fn from(s: &Script<T>) -> Self {
        Machine {
            ip: 0,
            script: s.clone(),
            data_stack: Stack::new(),
            exe_stack: vec![s.len()]
        }
    }

    pub fn reboot(&mut self) {
        self.data_stack.clear();
        self.exe_stack.clear();
        self.ip = 0;
    }

    pub fn load(&mut self, s: &Script<T>) {
        self.script = s.clone();
        self.exe_stack.push(s.len());
    }

    pub fn get_stack_mut(&mut self) -> &mut Stack<T> {
        &mut self.data_stack
    }

    pub fn push(&mut self, t: T) {
        self.data_stack.push(t);
    }

    pub fn pop(&mut self) -> T {
        match self.data_stack.pop() {
            Some(t) => t,
            _ => panic!()
        }
    }

    pub fn get_ip(&self) -> usize {
        self.ip
    }

    pub fn next_ip(&self) -> Option<usize> {
        Some(self.ip + 1)
    }

    pub fn push_frame(&mut self, ret: usize) {
        self.exe_stack.push(ret);
    }

    pub fn pop_frame(&mut self) -> Option<usize> {
        self.exe_stack.pop()
    }

    pub fn get_instruction(&self, ip: usize) -> Option<T> {
        self.script.get(ip)
    }

    pub fn execute(&mut self) -> Option<Stack<T>> {
        loop {
            match self.get_instruction(self.get_ip()) {
                Some(i) => {
                    self.ip = match i.execute(self) {
                        Some(new_ip) => new_ip,

                        // something went wrong in execution so halt and do not
                        // return the stack
                        None => return None
                    }
                },
                None => {
                    // we reached the end of the script, return the stack
                    return Some(self.data_stack.clone());
                }
            }
        }
    }
}

