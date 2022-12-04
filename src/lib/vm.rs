use std::collections::HashMap;
use std::str::FromStr;

use crate::config_ast;
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };

pub type Ast = Vec<config_ast::Expr>;
#[derive(Debug, Clone)]
pub enum OpCode {
    Int(i32),
    Str(String),
    StoreVar(usize),
    LoadVar(usize),
    Plus,
    Minus,
}

fn compiler(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Result<Vec<OpCode>, String> {
    // assert!(ast.len() >= 1);
    // dbg!(compiler_expr(&ast[0], lexer))
    if ast.is_empty() {
        return Ok(Vec::new());
    }

    let mut res = Vec::new();
    for node in ast {
        let val = compiler_expr(&node, lexer);
        res.extend(val);
    }
    Ok(res)
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Vec<OpCode> {
    match node {
        config_ast::Expr::Int { span: _, is_negative, val } => {
            println!("entering Expr::Int");
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            let mut res = Vec::new();
            if *is_negative {
                tmp = -1 * tmp;
            }
            res.push(OpCode::Int(tmp));
            res
        }
        config_ast::Expr::Assign { span: _, id, expr } => {
            println!("entering assignment");
            let mut res = Vec::new();
            let val = compiler_expr(expr, lexer);
            let idx = lexer.span_str(*id);

            res.extend(val.clone());
            println!("here is val {:?}", val);
            println!("here is idx {}", idx);
            //res.push(OpCode::StoreVar(idx));
            res
        }
        config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
            println!("entering binary op");
            let lhs = compiler_expr(lhs, lexer);
            let rhs = compiler_expr(rhs, lexer);
            let _op = lexer.span_str(*op);
            match _op {
                "+" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Plus);
                    println!("res is : {:?}", res);
                    return res;
                }
                "-" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Minus);
                    println!("res is : {:?}", res);
                    return res;
                }
                &_ => todo!(),
            }
        }
        config_ast::Expr::String(..) => {
            //todo
            unimplemented!();
        }
        config_ast::Expr::VarLookup(id) => {
            let mut res = Vec::new();
            let idx = usize::from_str(lexer.span_str(*id)).unwrap_or(0);
            res.push(OpCode::LoadVar(idx));
            res
        }
    }
}

fn vm(prog: Vec<OpCode>) -> Result<i32, String> {
    if prog.is_empty() {
        return Err("Cannot execute empty program".to_string());
    }
    let mut pc = 0;
    let mut stack: Vec<i32> = Vec::new();
    let mut locals: HashMap<usize, i32> = HashMap::new();
    while pc < prog.len() {
        let expr = &prog[pc];

        match *expr {
            OpCode::Int(x) => {
                stack.push(x);
                pc += 1;
            }
            OpCode::Str(..) => {
                // stack.push(OpCode::Str(val.clone()));
                // pc += 1;
                todo!();
            }
            OpCode::StoreVar(name) => {
                let val = stack.pop().unwrap();
                locals.insert(name, val);
                pc += 1;
            }
            OpCode::LoadVar(name) => {
                let val = locals.get(&name).unwrap();
                stack.push(*val);
                pc += 1;
            }
            OpCode::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
                pc += 1;
            }
            OpCode::Minus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
                pc += 1;
            }
        }
    }
    let l = stack.len();
    println!("here is e: {l}");
    //assert_eq!(stack.len(), 1);
    Ok(stack.pop().unwrap())
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> i32 {
    let prog = compiler(ast, lexer);

    match prog {
        Ok(prog_val) => {
            let result = vm(prog_val);
            match result {
                Ok(result_val) => {
                    return result_val;
                }
                Err(err) => {
                    println!("error: {}", err);
                    return 0;
                }
            }
        }
        Err(err) => {
            println!("error: {}", err);
            return 0;
        }
    }
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
    //#[test]
    // fn basic() {
    //     assert_eq!(compile_and_run("2+3;"), 5);
    //     assert_eq!(compile_and_run("2+3+4;"), 9);
    //     assert_eq!(compile_and_run("2 + -3;"), -1);
    //     assert_eq!(compile_and_run("2 - 3"), -1);
    // }
    #[test]
    fn test2() {
        assert_eq!(compile_and_run("let x = 1+2"), 3);
    }
}