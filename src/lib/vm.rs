//use std::collections::HashMap;

use crate::config_ast;
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };

#[derive(Debug)]
pub enum OpCode {
    Int(i32),
    Str(String),
    StoreVar(usize),
    LoadVar(usize),
    Plus,
}

pub type Ast = Vec<config_ast::Expr>;

// pub struct Compiler<'a> {
//     _ast: Ast,
//     lexer: &'a dyn NonStreamingLexer<'a, DefaultLexeme<u32>, u32>,
//     locals_stack: Vec<HashMap<String, usize>>,
// }

// impl<'a> Compiler<'a> {
//     fn new(lexer: &'a dyn NonStreamingLexer<'a, DefaultLexeme<u32>, u32>, _ast: Ast) -> Compiler {
//         Compiler { _ast: _ast, lexer: lexer, locals_stack: Vec::new() }
//     }
// }

fn compiler(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<OpCode> {
    let mut bc: Vec<OpCode> = Vec::new();
    assert!(ast.len() == 1);
    dbg!(compiler_expr(&ast[0], lexer))
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Vec<OpCode> {
    match node {
        config_ast::Expr::Int { span: _, is_negative, val } => {
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            if *is_negative {
                tmp = -1 * tmp;
            }
            vec![OpCode::Int(tmp)]
        }
        config_ast::Expr::Assign { span: _, id, expr } => {
            //todo
            // let res = self.eval(lexer, expr);
            // let _id = lexer.span_str(*id);

            // println!("{_id} is {res}");
            // res;
            todo!();
        }
        config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
            //todo
            let lhs = compiler_expr(lhs, lexer);
            let rhs = compiler_expr(rhs, lexer);
            let _op = lexer.span_str(*op);
            match _op {
                "+" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Plus);
                    res
                }
                //"-" => lhs - rhs,
                &_ => todo!(),
            }
        }
        config_ast::Expr::String(..) => {
            //todo
            unimplemented!();
        }
        config_ast::Expr::VarLookup(..) => {
            //todo
            unimplemented!();
        }
    }

    // for node in ast {
    //     let val = evaluator.eval(lexer, &node);
    //     bc.push(OpCode::Int(val));
    // }
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
        //locals: Vec<HashMap<String, usize>>
        //ctx: &mut Compiler
    ) -> i32 {
        match node {
            config_ast::Expr::Int { span: _, is_negative, val } => {
                let mut tmp = lexer.span_str(*val).parse().unwrap();
                if *is_negative {
                    tmp = -1 * tmp;
                }
                tmp
            }
            config_ast::Expr::Assign { span: _, id, expr } => {
                //todo
                let res = self.eval(lexer, expr);
                let _id = lexer.span_str(*id);

                println!("{_id} is {res}");
                res
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

fn vm(prog: Vec<OpCode>) -> i32 {
    let mut pc = 0;
    let mut stack: Vec<i32> = Vec::new();
    while pc < prog.len() {
        match *&prog[pc] {
            OpCode::Int(x) => {
                stack.push(x);
                pc += 1;
            }
            OpCode::Str(..) => {
                // stack.push(OpCode::Str(val.clone()));
                // pc += 1;
                todo!();
            }
            OpCode::StoreVar(..) => {
                // stack.push(OpCode::StoreVar(name));
                // pc += 1;
            }
            OpCode::LoadVar(name) => {}
            OpCode::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
                pc += 1;
            }
        }
    }
    assert_eq!(stack.len(), 1);
    stack[0]
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> i32 {
    let prog = compiler(ast, lexer);
    vm(prog)
}

#[cfg(test)]
mod test {
    use crate::mainv2::ukiyo_l;
    use crate::mainv2::ukiyo_y;
    use crate::vm::run;

    fn compile_and_run(input: &str) -> i32 {
        let lexerdef = ukiyo_l::lexerdef();
        let lexer = lexerdef.lexer(input);
        let res = ukiyo_y::parse(&lexer).0.unwrap().unwrap();
        run(res, &lexer)
    }
    #[test]
    fn basic() {
        assert_eq!(compile_and_run("2+3;"), 5);
        assert_eq!(compile_and_run("2+3+4;"), 9);
        assert_eq!(compile_and_run("2 + -3;"), -1);
    }
}