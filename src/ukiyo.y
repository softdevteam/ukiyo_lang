%start prog
%%
prog -> Result<Vec<Statement>, ()>: 
            prog "SEMICOLON" statement { flattenr($1, $3) }
          | statement { Ok(vec![]) }
          ;

statement -> Result<Statement, ()>: 
            expression { $1 }
          | assigment { $1 }
          ;

expression -> Result<Expr, ()>:
            unit { $1 }
          | binary_expression { $1 }
          ;

assigment -> Result<Statement, ()>: 
          "LET" "IDENTIFIER" "EQ" expression {  
            Ok(())
            };

unit -> Result<Expr, ()>:
        "IDENTIFIER" { Ok(Expr::VarLookup(map_err($1)?)) }
      | literal { $1 }
      | "LBRACK" expression "RBRACK" { $2 }
      ;
          
literal -> Result<Expr, ()>: 
          "INT" { Ok(Expr::Int{ span: $span, is_negative: false, val: map_err($1)? }) }
        | "MINUS" "INT" { Ok(Expr::Int{ span: $span, is_negative: true, val: map_err($2)? }) }
        | "STRING" { Ok(Expr::String(map_err($1)?)) }
        ;

binary_expression -> Result<Expr, ()>: 
                    binary_expression bin_op binary_term { Ok(Expr::BinaryOp { span: $span, op: $2?, lhs: Box::new($1?), rhs: Box::new($3?)} ) }
                  | binary_term { $1 }
                  ;

binary_term -> Result<(), Box<dyn Error>>:
               unit idlistOpt { Ok(()) };

idlistOpt -> Result<Vec<Span>, ()>:
            idlist { $1 };

idlist -> Result<Vec<Span>, ()>:
          "IDENTIFIER" { Ok(vec![map_err($1)?]) }
        | idlist "IDENTIFIER" { Ok(vec![]) }
        ;      

bin_op -> Result<Span, ()>: 
           "PLUS"  { Ok(map_err($1)?) }
        | "MINUS" { Ok(map_err($1)?) }
        | "LTEQ"  { Ok(map_err($1)?) }
        | "GTEQ"  { Ok(map_err($1)?) }
        | "LT"    { Ok(map_err($1)?) }
        | "GT"    { Ok(map_err($1)?) }
        | "EQEQ"  { Ok(map_err($1)?) }
        ;
%%

pub struct Prog(Vec<Statement>);

use crate::config_ast::{Statement, Expr};
use std::error::Error;
use lrlex::DefaultLexeme;
use lrpar::Span;

type StorageT = u32;

fn map_err(r: Result<DefaultLexeme<StorageT>, DefaultLexeme<StorageT>>)
    -> Result<Span, ()>
{
    r.map(|x| x.span()).map_err(|_| ())
}

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}