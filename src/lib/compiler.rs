use crate::config_ast::{ self };
use lrlex::DefaultLexeme;
use lrpar::{ NonStreamingLexer };
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
    DefineFunc(String, Vec<OpCode>),
}

pub fn compiler(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
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
    bc: &Vec<OpCode>
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
        config_ast::Expr::Assign { span: _, ref id, ref expr } => {
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
            println!("In print call");
            res.push(OpCode::Call(label));
            res
        }
        config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
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
        config_ast::Expr::WhileLoop { span: _, condition, body } => {
            let mut res = Vec::new();
            let loop_entry = bc.len();
            //getting and pushing the condition
            let cond = compiler_expr(condition, lexer, locals, bc);
            res.extend(cond);
            //println!("bytecode length is: {}", loop_entry);
            //if condition fails then jump to offset MAXD
            res.push(OpCode::JumpIfFalse(MAXD));
            let exit_call = bc.len() + res.len() - 1;
            //getting body and pushing it to res
            let body = compiler_expr(body, lexer, locals, bc);
            res.extend(body);
            //
            res.push(OpCode::Jump(loop_entry));
            res.push(OpCode::JumpIfFalse(exit_call));
            res
        }
        config_ast::Expr::IfStatement { span: _, condition, body } => {
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
        config_ast::Expr::FuncDef { span: _, name, args_list, body } => {
            let mut res = Vec::new();
            let mut new_locals = Vec::new();
            let mut func_body = Vec::new();

            for arg in args_list.iter() {
                let idx_str = lexer.span_str(*arg).to_string();
                println!("arg is : {}", idx_str);
                new_locals.push(idx_str);
            }
            println!("new local is: {:?}", new_locals);
            let body = compiler_expr(body, lexer, &mut new_locals, bc);
            println!("body is {:?}", body);
            func_body.extend(body);
            let func_name = lexer.span_str(*name).to_string();
            println!("function name is {} and body is {:?}", func_name, func_body);
            res.push(OpCode::DefineFunc(func_name, func_body));
            println!("res is {:?}", res);
            res
        }

        config_ast::Expr::Call { span, name } => {
            let mut res = Vec::new();
            let func_name = lexer.span_str(*name).to_string();
            res.push(OpCode::Call(func_name));
            res
        }
    }
}