use crate::config_ast::{Expr};
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
      Expr::Int(val) => *val,
      Expr::Assign { span, id, expr } => {
        //todo
        unimplemented!();
      },
      Expr::BinaryOp { span, op, lhs, rhs } => {
        //todo
        unimplemented!();
      },
      Expr::String(span) => {
        //todo
        unimplemented!();
      },
      Expr::VarLookup(span) => {
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