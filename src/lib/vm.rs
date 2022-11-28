use crate::config_ast;
use lrlex::{DefaultLexeme};
use lrpar::{NonStreamingLexer};
pub enum OpCode {
    Int(i32),
}

pub type Ast = Vec<config_ast::Expr>;

fn compiler(_ast: Ast,  lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,) -> Vec<OpCode> {
  let mut bc: Vec<OpCode> = Vec::new();
  let evaluator = Eval::new();
  for node in _ast {
    let val = evaluator.eval(lexer, &node);
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
  pub fn eval(&self,  lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>, node: &config_ast::Expr) -> i32 {
    match node {
      config_ast::Expr::Int(val) => {
        let tmp = lexer.span_str(*val);
        tmp.parse().unwrap()
      }
      config_ast::Expr::Assign { .. } => {
        //todo
        unimplemented!();
      },
      config_ast::Expr::BinaryOp { span: _, op: _, lhs, rhs } => {
        //todo
        let lhs = self.eval(lexer, lhs);
        let rhs = self.eval(lexer, rhs);
        // let op = 
        lhs+rhs
      },
      config_ast::Expr::String(..) => {
        //todo
        unimplemented!();
      },
      config_ast::Expr::VarLookup(..) => {
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

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) {
  let prog = compiler(ast, lexer);
  vm(prog);
}