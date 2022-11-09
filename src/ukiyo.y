%start prog
%%
prog -> Result<Vec<Statement>, ()>: 
            prog "SEMICOLON" statement { Ok() }
          | statement { Ok() }
          ;

statement -> Result<Statement, ()>: 
            expression { Ok() }
          | assigment { Ok() }
          ;

assigment -> Result<Expr, ()>: 
          "LET" "IDENTIFIER" "EQ" expression { Ok() };

expression -> Result<Expr, ()>:
            variable { Ok() }
          // | binary_expression { Ok() }
          | literal { Ok() }
          ;

variable -> Result<Expr::Var, ()>: 
           "IDENTIFIER" { Ok(Expr::Var) };

// binary_expression -> Result<BinOp, ()>: 
//            expression bin_op expression { Ok() };

bin_op -> Result<Span, ()>: 
          "PLUS"  { Ok() }
        | "MINUS" { Ok() }
        | "LTEQ"  { Ok() }
        | "GTEQ"  { Ok() }
        | "LT"    { Ok() }
        | "GT"    { Ok() }
        | "EQEQ"  { Ok() }
        ;

literal -> Result<Expr, ()>: 
          "INT" { Ok(Expr::Int{ span: $span, is_negative: false, val: map_err($1)?.span() }) };
%%

pub struct Prog(Vec<statement>);
use crate::compiler::ast::{Statement, Expr, BinOp};