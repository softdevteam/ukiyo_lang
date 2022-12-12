use crate::config_ast;
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };

pub type Ast = Vec<config_ast::Expr>;
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
    Jump(String),
    JumpIfFalse(String),
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
        config_ast::Expr::Assign { span: _, id, expr } => {
            let mut res = Vec::new();
            let val = compiler_expr(expr, lexer, locals);
            let idx_str = lexer.span_str(*id).to_string();
            println!("we now in assignment call for var: {}", idx_str);
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
        config_ast::Expr::VarLookup(id) => {
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
            let condition_val = compiler_expr(condition, lexer, locals);
            res.extend(condition_val);
            let start_label = "while_loop_start".to_string();
            res.push(OpCode::Label(start_label.clone()));

            res.push(OpCode::JumpIfFalse("while_loop_end".to_string()));

            let body_val = compiler_expr(body, lexer, locals);
            res.extend(body_val);

            res.push(OpCode::Jump(start_label.clone()));

            let end_label = "while_loop_end".to_string();
            res.push(OpCode::Label(end_label));
            println!("inside while loop now, res is: {:?}", res);
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