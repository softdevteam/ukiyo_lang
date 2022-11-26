use crate::config_ast::{Expr};
pub enum OpCode {
    Int(i32),
}

pub type Ast = Vec<Expr>;

fn compiler(ast: Ast) -> Vec<OpCode> {
  let mut bc: Vec<OpCode> = Vec::new();
  // for node in ast {
  //   bc.push(OpCode::Int(node));
  // }
  bc.push(OpCode::Int(2));
  bc.push(OpCode::Int(3));
  bc
}

fn vm(prog: Vec<OpCode>) {
  let mut pc = 0;
  let mut stack = Vec::new();
  while pc < prog.len() {
    match prog[pc] {
      OpCode::Int(x) => { stack.push(x); pc += 1; }
    }
  }
  println!("{stack:?}");
}

pub fn run(ast: Ast) {
  let prog = compiler(ast);
  vm(prog);
}