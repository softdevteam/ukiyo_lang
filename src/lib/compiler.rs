use crate::config_ast::{self};
use lrlex::DefaultLexeme;
use lrpar::NonStreamingLexer;
use std::fmt;
pub type Ast = Vec<config_ast::Expr>;
const MAXD: usize = usize::max_value();
#[derive(Debug, Clone)]
pub enum OpCode {
    PushInt(i32),
    PushStr(String),
    Plus,
    Minus,
    Eqeq,
    Lteq,
    Lt,
    StoreVar(usize),
    LoadVar(usize),
    Call(String),
    Jump(usize),
    JumpIfFalse(usize),
    Return,
    DefineFunc(String, Vec<String>, Vec<OpCode>),
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::PushInt(i) => write!(f, "PushInt({})", i),
            OpCode::PushStr(s) => write!(f, "PushStr({})", s),
            OpCode::Plus => write!(f, "Plus"),
            OpCode::Minus => write!(f, "Minus"),
            OpCode::Eqeq => write!(f, "Eqeq"),
            OpCode::Lteq => write!(f, "Lteq"),
            OpCode::Lt => write!(f, "Lt"),
            OpCode::StoreVar(i) => write!(f, "StoreVar({})", i),
            OpCode::LoadVar(i) => write!(f, "LoadVar({})", i),
            OpCode::Call(s) => write!(f, "Call({})", s),
            OpCode::Jump(i) => write!(f, "Jump({})", i),
            OpCode::JumpIfFalse(i) => write!(f, "JumpIfFalse({})", i),
            OpCode::Return => write!(f, "Return"),
            OpCode::DefineFunc(s, ops1, ops2) => {
                write!(f, "DefineFunc({}, {:?}, {:?})", s, ops1, ops2)
            }
        }
    }
}
pub fn compiler(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
) -> Result<Vec<OpCode>, String> {
    if ast.is_empty() {
        return Ok(Vec::new());
    }

    let mut bc = Vec::new();
    let mut locals: Vec<String> = Vec::new();

    for node in ast {
        let val = compiler_expr(&node, lexer, &mut locals, &bc);
        bc.extend(val);
    }
    Ok(bc)
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
    locals: &mut Vec<String>,
    bc: &Vec<OpCode>,
) -> Vec<OpCode> {
    match node {
        config_ast::Expr::Int {
            span: _,
            is_negative,
            val,
        } => {
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            let mut res = Vec::new();
            if *is_negative {
                tmp = -1 * tmp;
            }
            res.push(OpCode::PushInt(tmp));
            res
        }
        config_ast::Expr::String(span) => {
            let mut res = Vec::new();
            let s_orig = lexer.span_str(*span);
            let mut new_s = String::new();
            // Start by ignoring the beginning quote.
            let mut i = '\"'.len_utf8();
            // End by ignoring the beginning quote.
            while i < s_orig.len() - '\"'.len_utf8() {
                let mut c = s_orig[i..].chars().next().unwrap();
                if c == '\\' {
                    i += c.len_utf8();
                    let next_c = s_orig[i..].chars().next().unwrap();
                    c = match next_c {
                        't' => '\t',
                        'b' => '\x08',
                        'n' => '\n',
                        'r' => '\r',
                        'f' => '\x0C',
                        '\'' => '\'',
                        '\\' => '\\',
                        '0' => '\0',
                        _ => next_c,
                    };
                }
                new_s.push(c);
                i += c.len_utf8();
            }
            //SPLIT STRING AFTER TAKING THE MATCHING CLOSING `"`
            // let val = lexer.span_str(*span).parse().unwrap();

            res.push(OpCode::PushStr(new_s));
            res
        }
        config_ast::Expr::Assign {
            span: _,
            ref id,
            ref expr,
        } => {
            let mut res = Vec::new();
            let val = compiler_expr(&expr, lexer, locals, bc);
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
            let val = compiler_expr(&args, lexer, locals, bc);
            res.extend(val);
            res.push(OpCode::Call(label));
            res
        }
        config_ast::Expr::BinaryOp {
            span: _,
            op,
            lhs,
            rhs,
        } => {
            let lhs = compiler_expr(lhs, lexer, locals, bc);
            let rhs = compiler_expr(rhs, lexer, locals, bc);
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
                "==" => {
                    let mut res = Vec::new();
                    res.extend(lhs);
                    res.extend(rhs);
                    res.push(OpCode::Eqeq);
                    return res;
                }
                &_ => todo!(),
            }
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
        config_ast::Expr::WhileLoop {
            span: _,
            condition,
            body,
        } => {
            let mut res = Vec::new();
            let loop_entry = bc.len();

            let cond = compiler_expr(condition, lexer, locals, bc);
            res.extend(cond);

            res.push(OpCode::JumpIfFalse(MAXD));
            let exit_call = bc.len() + res.len() - 1;

            let body = compiler_expr(body, lexer, locals, bc);
            res.extend(body);

            res.push(OpCode::Jump(loop_entry));
            res.push(OpCode::JumpIfFalse(exit_call));
            res
        }
        config_ast::Expr::IfStatement {
            span: _,
            condition,
            body,
        } => {
            let mut res = Vec::new();
            let cond = compiler_expr(condition, lexer, locals, bc);
            res.extend(cond);
            res.push(OpCode::JumpIfFalse(MAXD));
            let body = compiler_expr(body, lexer, locals, bc);
            res.extend(body);
            res
        }
        config_ast::Expr::Prog { span: _, stmts } => {
            let mut res = Vec::new();
            for stmt in stmts {
                res.extend(compiler_expr(stmt, lexer, locals, bc));
            }
            res
        }
        config_ast::Expr::FuncDef {
            span: _,
            name,
            args_list,
            body,
        } => {
            let mut res = Vec::new();
            let mut new_locals = Vec::new();
            let mut args = Vec::new();
            let mut func_body = Vec::new();
            let func_name = lexer.span_str(*name).to_string();

            for arg in args_list {
                let val = lexer.span_str(*arg).to_string();
                new_locals.push(val);
            }
            args.extend(new_locals.clone());
            let body = compiler_expr(body, lexer, &mut new_locals, bc);
            func_body.extend(body);

            res.push(OpCode::DefineFunc(func_name.clone(), args, func_body));
            res
        }

        config_ast::Expr::Call {
            span: _,
            name,
            params,
        } => {
            let mut res = Vec::new();

            for param in params {
                let val = compiler_expr(param, lexer, locals, bc);
                res.extend(val);
            }
            let func_name = lexer.span_str(*name).to_string();
            res.push(OpCode::Call(func_name));
            res
        }

        config_ast::Expr::Return { span: _, expr } => {
            let mut res = Vec::new();
            res.extend(compiler_expr(&*expr, lexer, locals, bc));
            res.push(OpCode::Return);
            res
        }
    }
}
