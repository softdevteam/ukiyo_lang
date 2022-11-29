use crate::config_ast;
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };

#[derive(Debug)]
pub enum OpCode {
    Int(i32),
    Str(String),
}

pub type Ast = Vec<config_ast::Expr>;

fn compiler(_ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<OpCode> {
    let mut bc: Vec<OpCode> = Vec::new();
    let evaluator = Eval::new();
    for node in _ast {
        let val = evaluator.eval(lexer, &node);
        bc.push(OpCode::Int(val));
    }
    bc
}

struct Eval;
impl Eval {
    fn new() -> Self {
        Self
    }
    pub fn eval(
        &self,
        lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
        node: &config_ast::Expr
    ) -> i32 {
        match node {
            config_ast::Expr::Int { span: _, is_negative, val } => {
                let mut tmp = lexer.span_str(*val).parse().unwrap();
                if *is_negative {
                    tmp = -1 * tmp;
                }
                tmp
            }
            config_ast::Expr::Assign { .. } => {
                //todo
                // let res = self.eval(lexer, expr);
                // let _id = lexer.span_str(*id);
                // println!("{_id} is {res}");
                // res
                unimplemented!();
            }
            config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
                //todo
                let lhs = self.eval(lexer, lhs);
                let rhs = self.eval(lexer, rhs);
                let _op = lexer.span_str(*op);
                let res = match _op {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    &_ => todo!(),
                };
                res
            }
            config_ast::Expr::String(..) => {
                //todo
                unimplemented!()
            }
            config_ast::Expr::VarLookup(..) => {
                //todo
                unimplemented!();
            }
        }
    }
}

fn vm(prog: Vec<OpCode>) {
    let mut pc = 0;
    let mut stack: Vec<OpCode> = Vec::new();
    while pc < prog.len() {
        match *&prog[pc] {
            OpCode::Int(ref x) => {
                stack.push(OpCode::Int(x.clone()));
                pc += 1;
            }
            OpCode::Str(ref val) => {
                stack.push(OpCode::Str(val.clone()));
                pc += 1;
            }
        }
    }
    println!("{stack:?}");
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) {
    let prog = compiler(ast, lexer);
    vm(prog);
}