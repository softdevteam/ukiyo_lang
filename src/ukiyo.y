%start prog
%%
prog -> Result<(), ()>: prog "SEMICOLON" statement {Ok()}
     | statement {Ok()}
     ;

statement -> Result<(), ()>: expression {Ok()}
          | assigment {Ok()}
          ;

assigment -> Result<assigment, ()>: "LET" "IDENTIFIER" "EQ" expression {Ok()};

expression -> Result<expression, ()>: variable
           | binary_expression {Ok()}
           | literal {Ok()}
           ;

variable -> Result<String, ()>: "IDENTIFIER" {Ok(Expr::Var)};

binary_expression -> Result<binary_expression, ()>: expression bin_op expression {Ok()};

bin_op -> Result<String, ()>: "PLUS" {Ok()}
       | "MINUS" {Ok()}
       | "LTEQ" {Ok()}
       | "GTEQ" {Ok()}
       | "LT"   {Ok()}
       | "GT"   {Ok()}
       | "EQEQ" {Ok()}
       ;

literal -> Result<Expr, ()>:  | "INT" { Ok(Expr::Int{ span: $span, is_negative: false, val: map_err($1)?.span() }) };

%%
pub struct Prog(Vec<statement>);
use crate::config_ast::{Statement, Expr, BinOp};