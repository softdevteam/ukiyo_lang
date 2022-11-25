use crate::Node;
use parse::Instr;
use parse::Bytecode;
use std::collections::HashMap;

const STACKSIZE: usize = usize::max_value();

#[derive(Debug, Clone)]
pub enum NativeType {
    Int(i32),
    Str(String),
    NoneType,
}

impl NativeType {
    fn pretty(&self) -> String {
        match *self {
            NativeType::Int(ref x) => x.to_string(),
            NativeType::Str(ref x) => x.to_string(),
            NativeType::NoneType => "None".to_string()
        }
    }
}

pub struct VM {
    bytecode: Bytecode,
    stack: [Node; STACK_SIZE],
    stack_ptr: usize,
    locals: HashMap<String, NativeType>,
    name: String,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self{
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0,
            locals: locals,
            name: name,
        }
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.bytecode.bytecode.len() {
            let instr_addr = ip;
            ip += 1;

            match *&self.bytecode.bytecode[instr_addr] {
                Instr::PushInt(ref x) => {
                    let const_idx =self.bytecode.bytecode[ip];
                    self.push(self.bytecode.constants[NativeType::Int(const_idx)].clone());
                }
                Instr::PushStr(ref x) => {
                    let const_idx =self.bytecode.bytecode[ip];
                    self.push(self.bytecode.constants[NativeType::Int(const_idx)].clone());
                }
                Instr::Pop => {
                    self.pop();
                }
                Instr::Add => {
                     match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs + rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::LoadVar(index) => {
                    self.load_local(index);
                }
                Instr::StoreVar(name) => {
                    self.store_local(name);
                }
            }
        }
    }

    fn push(&mut self, obj: NativeType) {
        self.stack[self.stack_ptr] = obj;
        self.stack_ptr += 1;
    }

    fn pop(&mut self) -> NativeType {
        let node = self.stack[self.stack_ptr - 1].clone();
        match node {
            Some(x) => x,
            None => panic!("Popped from empty stack!"),
        }
    }
    fn load_local(&mut self, index: usize) {
        let value = self.locals[index].clone();
        self.push(value)
    }
    fn store_local(&mut self, index: usize) {
        let value = self.pop();
        let len = self.locals.len();
        if index < len {
            self.locals[index] = value;
        }
        else {
            assert_eq!(index, len);
            self.locals.push(value)
        }
    } 
}

pub fn run(bytecode: Bytecode) -> String {
    let mut vm = VM::new(bytecode);
    let res = vm.run();
    match res {
        Some(ref x) => x.pretty(),
        None => "".to_string(),
    }
}