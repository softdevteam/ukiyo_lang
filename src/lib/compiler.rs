use crate::config_ast::{ self };
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };
pub type Ast = Vec<config_ast::Expr>;
const PLACEHOLDER: usize = usize::max_value();
#[derive(Debug, Clone)]
pub enum OpCode {
    PushInt(i32),
    PushStr(String),
    Plus,
    Minus,
    Lteq,
    Lt,
    StoreVar(usize),
    LoadVar(usize),
    Call(String),
    Jump(usize),
    JumpIfFalse(usize),
    Label(String),
}

pub fn compiler(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Result<Vec<OpCode>, String> {
    if ast.is_empty() {
        return Ok(Vec::new());
    }

    let mut res = Vec::new();
    let mut locals: Vec<String> = Vec::new();

    for node in ast {
        let val = compiler_expr(&node, lexer, &mut locals);
        res.extend(val);
    }
    Ok(res)
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
    locals: &mut Vec<String>
) -> Vec<OpCode> {
    let mut i = 0; //for iterating in while loop
    match node {
        config_ast::Expr::Int { span: _, is_negative, val } => {
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            let mut res = Vec::new();
            if *is_negative {
                tmp = -1 * tmp;
            }
            res.push(OpCode::PushInt(tmp));
            res
        }
        //error when "let a = 1; let b = 2; let a = b + 3; print(b); <- prints 1 instead of 2"
        config_ast::Expr::Assign { span: _, ref id, ref expr } => {
            let mut res = Vec::new();
            let val = compiler_expr(&expr, lexer, locals);
            let idx_str = lexer.span_str(*id).to_string();
            //println!("we now in assignment call for var: {}", idx_str);
            res.extend(val);
            match locals.iter().position(|x| x == &idx_str) {
                Some(x) => res.push(OpCode::StoreVar(x)),
                None => {
                    locals.push(idx_str);
                    res.push(OpCode::StoreVar(locals.len() - 1));
                }
            }
            res
        }
        config_ast::Expr::Print { span: _, args } => {
            let mut res = Vec::new();
            let label = "print".to_string();

            let args = &*args;
            let val = compiler_expr(&args, lexer, locals);
            res.extend(val);

            res.push(OpCode::Call(label));
            res
        }
        config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
            let lhs = compiler_expr(lhs, lexer, locals);
            let rhs = compiler_expr(rhs, lexer, locals);
            let _op = lexer.span_str(*op);
            match _op {
                "+" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Plus);
                    return res;
                }
                "-" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Minus);
                    return res;
                }
                "<" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Lt);
                    return res;
                }
                "<=" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Lteq);
                    return res;
                }
                &_ => todo!(),
            }
        }
        config_ast::Expr::String(..) => {
            //todo
            unimplemented!();
        }
        config_ast::Expr::VarLookup(ref id) => {
            let mut res = Vec::new();
            let idx_str = lexer.span_str(*id).to_string();
            let index = match locals.iter().position(|x| x == &idx_str) {
                Some(x) => x,
                None => {
                    panic!("Variable doesn't exists");
                }
            };
            res.push(OpCode::LoadVar(index));
            res
        }
        config_ast::Expr::WhileLoop { span: _, condition, body } => {
            let mut res = Vec::new();
            let loop_entry = res.len();
            //println!("loop_entry is {}", loop_entry);
            let condition_val = compiler_expr(&condition, lexer, locals);
            res.extend(condition_val);
            //Problem with JumpIfFalse
            // Test case: let z = 0; while (z < 1) { z < 1; print(z); let z = z + 1; print(z); }
            res.push(OpCode::JumpIfFalse(PLACEHOLDER));
            let mut body_val = Vec::new();
            for stmt in body {
                match stmt {
                    config_ast::Expr::Prog { span: _, stmts } => {
                        // Generate code for each statement in the program
                        for inner_stmt in stmts {
                            body_val.extend(compiler_expr(&inner_stmt, lexer, locals));
                        }
                    }
                    _ => {
                        body_val.extend(compiler_expr(stmt, lexer, locals));
                    }
                }
            }
            println!("printing body {:?}", body_val);
            //res.extend(body_val);
            //println!("at end of iteration {} loop enrty is {}", i, loop_entry);
            res.push(OpCode::Jump(loop_entry));

            let end_label = "while_loop_end".to_string();
            res.push(OpCode::Label(end_label));
            i = i + 1;
            //println!("inside while loop now, res is: {:?}", res);
            res
        }

        config_ast::Expr::Prog { span: _, stmts } => {
            let mut res = Vec::new();
            for stmt in stmts {
                res.extend(compiler_expr(stmt, lexer, locals));
            }
            res
        }
    }
}