use std::collections::HashMap;

use crate::config_ast;
use lrlex::{ DefaultLexeme };
use lrpar::{ NonStreamingLexer };

pub type Ast = Vec<config_ast::Expr>;
#[derive(Debug, Clone)]
pub enum OpCode {
    Int(i32),
    Str(String),
    Plus,
    Minus,
    Lteq,
    StoreVar(usize),
    LoadVar(usize),
}

pub fn compiler(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Result<Vec<OpCode>, String> {
    if ast.is_empty() {
        return Ok(Vec::new());
    }

    let mut res = Vec::new();
    let mut hash_map: HashMap<String, usize> = HashMap::new();

    for node in ast {
        let val = compiler_expr(&node, lexer, &mut hash_map);
        res.extend(val);
    }
    Ok(res)
}

fn compiler_expr(
    node: &config_ast::Expr,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
    hash_map: &mut HashMap<String, usize>
) -> Vec<OpCode> {
    match node {
        config_ast::Expr::Int { span: _, is_negative, val } => {
            let mut tmp = lexer.span_str(*val).parse().unwrap();
            let mut res = Vec::new();
            if *is_negative {
                tmp = -1 * tmp;
            }
            res.push(OpCode::Int(tmp));
            res
        }
        config_ast::Expr::Assign { span: _, id, expr } => {
            let mut res = Vec::new();
            let val = compiler_expr(expr, lexer, hash_map);
            let idx_str = lexer.span_str(*id).to_string();

            let map_len = hash_map.len();
            let _ = *hash_map.entry(idx_str).or_insert(map_len);
            res.extend(val.clone());
            res.push(OpCode::StoreVar(map_len));
            res
        }
        config_ast::Expr::BinaryOp { span: _, op, lhs, rhs } => {
            let lhs = compiler_expr(lhs, lexer, hash_map);
            let rhs = compiler_expr(rhs, lexer, hash_map);
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
            let index = hash_map.get(&idx_str).unwrap();
            res.push(OpCode::LoadVar(*index));
            res
        }
    }
}