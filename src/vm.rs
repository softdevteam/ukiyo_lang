use crate::config_ast::{Expr};
pub enum OpCode {
    Int(i32),
}

pub type Ast = Vec<Expr>;

pub struct Bytecode {
  pub bytecode: Vec<OpCode>,
}

impl Bytecode {
  fn new() -> Self {
    Self {
          bytecode: Vec::new(),
    }
  }
}
struct Compiler {
  bytecode: Bytecode,
}

impl Compiler(ast: Ast) -> Vec<OpCode> {
  fn new() -> Self {
    Self {
      bytecode: Vec::new(),
    }
  }
  fn gen_bc(&mut self , opcode: OpCode) -> usize {
        self.bytecode.push(opcode);
        self.bytecode.len() - 1
  }

  fn from_ast(ast: Ast) {
    for node in ast {
      println!("Compiling node {:?}", node);
      gen_bc(&node);
    }
  }
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

// pub fn run() {
//   let prog = compiler(ast);
//   VM(prog);
// }