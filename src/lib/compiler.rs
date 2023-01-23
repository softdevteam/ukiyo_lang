use crate::config_ast::{self};
use lrlex::DefaultLexeme;
use lrpar::NonStreamingLexer;
use std::fmt::{self};
pub type Ast = Vec<config_ast::Expr>;
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
    Patch,
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
            OpCode::Patch => write!(f, "Patch"),
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
        compiler_expr(&node, lexer, &mut locals, &mut bc);
    }
    Ok(bc)
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
    locals: &mut Vec<String>,
    bc: &mut Vec<OpCode>,
) {
    match node {
        config_ast::Expr::Int {
            span: _,
            is_negative,
            val,
        } => {
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            if *is_negative {
                tmp = -1 * tmp;
            }
            bc.push(OpCode::PushInt(tmp));
        }
        config_ast::Expr::String(span) => {
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

            bc.push(OpCode::PushStr(new_s));
        }
        config_ast::Expr::Assign {
            span: _,
            ref id,
            ref expr,
        } => {
            compiler_expr(&expr, lexer, locals, bc);
            let idx_str = lexer.span_str(*id).to_string();
            match locals.iter().position(|x| x == &idx_str) {
                Some(x) => bc.push(OpCode::StoreVar(x)),
                None => {
                    locals.push(idx_str);
                    bc.push(OpCode::StoreVar(locals.len() - 1));
                }
            }
        }
        config_ast::Expr::Print { span: _, args } => {
            let label = "print".to_string();

            let args = &*args;
            compiler_expr(&args, lexer, locals, bc);

            bc.push(OpCode::Call(label));
        }
        config_ast::Expr::BinaryOp {
            span: _,
            op,
            lhs,
            rhs,
        } => {
            compiler_expr(lhs, lexer, locals, bc);
            compiler_expr(rhs, lexer, locals, bc);
            let _op = lexer.span_str(*op);
            match _op {
                "+" => {
                    bc.push(OpCode::Plus);
                }
                "-" => {
                    bc.push(OpCode::Minus);
                }
                "<" => {
                    bc.push(OpCode::Lt);
                }
                "<=" => {
                    bc.push(OpCode::Lteq);
                }
                "==" => {
                    bc.push(OpCode::Eqeq);
                }
                &_ => todo!(),
            }
        }
        config_ast::Expr::VarLookup(ref id) => {
            let idx_str = lexer.span_str(*id).to_string();
            let index = match locals.iter().position(|x| x == &idx_str) {
                Some(x) => x,
                None => {
                    panic!("Variable doesn't exists");
                }
            };
            bc.push(OpCode::LoadVar(index));
        }
        config_ast::Expr::WhileLoop {
            span: _,
            condition,
            body,
        } => {
            let loop_entry = bc.len();
            compiler_expr(condition, lexer, locals, bc);
            bc.push(OpCode::Patch);
            let exit = bc.len() - 1;
            compiler_expr(body, lexer, locals, bc);

            compiler_expr(condition, lexer, locals, bc);
            bc.push(OpCode::Jump(loop_entry));

            bc[exit] = OpCode::JumpIfFalse(bc.len());
        }
        config_ast::Expr::IfStatement {
            span: _,
            condition,
            body,
        } => {
            compiler_expr(condition, lexer, locals, bc);
            bc.push(OpCode::Patch);
            let exit = bc.len() - 1;
            compiler_expr(body, lexer, locals, bc);
            bc[exit] = OpCode::JumpIfFalse(bc.len());
        }
        config_ast::Expr::Prog { span: _, stmts } => {
            for stmt in stmts {
                compiler_expr(stmt, lexer, locals, bc);
            }
        }
        config_ast::Expr::FuncDef {
            span: _,
            name,
            args_list,
            body,
        } => {
            let mut new_locals = Vec::new();
            let mut args = Vec::new();
            let mut func_body = Vec::new();
            let func_name = lexer.span_str(*name).to_string();

            for arg in args_list {
                let val = lexer.span_str(*arg).to_string();
                new_locals.push(val);
            }
            args.extend(new_locals.clone());
            bc.push(OpCode::Patch);

            let offset = bc.len() - 1;
            compiler_expr(body, lexer, &mut new_locals, &mut func_body);
            bc[offset] = OpCode::DefineFunc(func_name.clone(), args, func_body);
        }

        config_ast::Expr::Call {
            span: _,
            name,
            params,
        } => {
            for param in params {
                compiler_expr(param, lexer, locals, bc);
            }
            let func_name = lexer.span_str(*name).to_string();
            bc.push(OpCode::Call(func_name));
        }

        config_ast::Expr::Return { span: _, expr } => {
            compiler_expr(&*expr, lexer, locals, bc);
            bc.push(OpCode::Return);
        }
    }
}
