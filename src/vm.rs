use crate::config_ast::{Expr};
use std::convert::From;
pub enum OpCode {
    Int(i32),
}

pub type Ast = Vec<Expr>;

fn compiler(_ast: Ast) -> Vec<OpCode> {
  let mut bc: Vec<OpCode> = Vec::new();
  let evaluator = Eval::new();
  for node in _ast {
    let val = evaluator.eval(&node);
    bc.push(OpCode::Int(val));
  }
  // bc.push(OpCode::Int(2));
  // bc.push(OpCode::Int(3));
  bc
}

struct Eval;
impl Eval {
  fn new() -> Self {
    Self
  }
  pub fn eval(&self, node: &Expr) -> i32 {
    match node {
      Expr::Int(val) => {
        // *val as i32,
        let tmp: String = String::from(*val);
        let rval: i32 = tmp.trim_end().parse().unwrap();
        rval
      }
      Expr::Assign { .. } => {
        //todo
        unimplemented!();
      },
      Expr::BinaryOp { .. } => {
        //todo
        unimplemented!();
      },
      Expr::String(..) => {
        //todo
        unimplemented!();
      },
      Expr::VarLookup(..) => {
        //todo
        unimplemented!();
      },
  }
}
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