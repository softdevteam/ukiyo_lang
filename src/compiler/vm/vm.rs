use crate::Node;

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
    locals: Vec<NativeType>,
    name: String,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self{
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0,
        }
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.bytecode.instructions.len() {
            let instr_addr = ip;
            ip += 1;

            match *&self.bytecode.instructions[instr_addr] {
                Instr::PushInt(ref x) => {
                    let const_idx =self.bytecode.instructions[ip];
                    self.push(self.bytecode.constants[NativeType::Int(const_idx)].clone());
                }
                Instr::PushStr(ref x) => {
                    let const_idx =self.bytecode.instructions[ip];
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
                Instr::Sub => {
                     match (self.pop(), self.pop()) {
                        (Node::Int(rhs), Node::Int(lhs)) => self.push(Node::Int(lhs - rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::Lteq => {
                    match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs <= rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::Lt => {
                    match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs < rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::Gteq => {
                    match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs >= rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::Gt => {
                    match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs > rhs)),
                        _ => panic!("Unknown types"),
                    }
                }
                Instr::Eqeq => {
                    match (self.pop(), self.pop()) {
                        (NativeType::Int(rhs), NativeType::Int(lhs)) => self.push(NativeType::Int(lhs == rhs)),
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