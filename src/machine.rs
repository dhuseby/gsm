use crate::{
    AppIO,
    Instruction,
    Script,
    Stack
};
use semver::{
    Version,
    VersionReq
};
use std::convert::From;

pub struct MachineBuilder<I: Clone>
{
    s: Script<I>,
    v: VersionReq
}

impl<I: Clone + Instruction<I>> MachineBuilder<I> {
    pub fn new() -> Self {
        Self {
            s: Script::from(Vec::new()),
            v: VersionReq::any()
        }
    }

    pub fn script(&mut self, s: &Script<I>) -> &mut Self {
        self.s = s.clone();
        self
    }

    pub fn version_req(&mut self, v: &VersionReq) -> &mut Self {
        self.v = v.clone();
        self
    }

    pub fn build(&self) -> Machine<I> {
        Machine::new(&self.s, &self.v)
    }
}

pub struct Machine<I: Clone>
{
    v: VersionReq,
    d: Stack<I>,
    r: Stack<usize>,
    s: Script<I>
}

impl<I: Clone + Instruction<I>> Machine<I>
{
    fn new(s: &Script<I>, v: &VersionReq) -> Self {
        Self {
            v: v.clone(),
            d: Stack::<I>::new(),
            r: Stack::from(vec![0]),
            s: s.clone()
        }
    }

    pub fn push(&mut self, i: I) {
        self.d.push(i);
    }

    pub fn pop(&mut self) -> Option<I> {
        self.d.pop()
    }

    pub fn pushr(&mut self, i: usize) {
        self.r.push(i);
    }

    pub fn popr(&mut self) -> Option<usize> {
        self.r.pop()
    }

    pub fn geti(&self, i: usize) -> Option<I> {
        self.s.get(i)
    }

    pub fn version_check(&self, v: &Version) -> bool {
        self.v.matches(v)
    }

    pub fn reset(&mut self) {
        self.d = Stack::<I>::new();
        self.r = Stack::<usize>::new();
        self.pushr(0);
    }

    pub fn execute(&mut self, io: &dyn AppIO<I>) -> Option<Stack<I>>
    {
        loop {
            if let Some(ip) = self.popr() {
                if let Some(instr) = self.geti(ip) {
                    instr.execute(ip, self, io);
                } else {
                    // end of script
                    return Some(self.d.clone());
                }
            } else {
                return None;
            }
        }
    }
}

impl<I: Clone> From<Script<I>> for Machine<I> {
    fn from(s: Script<I>) -> Self {
        Machine {
            v: VersionReq::any(),
            d: Stack::<I>::new(),
            r: Stack::from(vec![0]),
            s: s
        }
    }
}
