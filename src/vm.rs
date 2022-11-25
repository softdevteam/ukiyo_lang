// use crate::config_ast;
pub enum OpCode {
    Int(i32),
}

fn compiler() -> Vec<OpCode> {
  vec![OpCode::Int(2)]
}

fn VM(prog: Vec<OpCode>) {
  let mut pc = 0;
  let mut stack = Vec::new();
  while pc < prog.len() {
    match prog[pc] {
      OpCode::Int(x) => { stack.push(x); pc += 1; }
    }
  }
  println!("{stack:?}");
}

pub fn run() {
  let prog = compiler();
  VM(prog);
}