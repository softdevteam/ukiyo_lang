%start prog

%%
prog -> prog "SEMICOLON" statement
     | statement
     ;

statement -> expression
          | assigment
          ;


assigment -> "LET" "IDENTIFIER" "EQ" expression;

expression -> variable
           | binary_expression
           | literal
           ;

variable -> "IDENTIFIER";

binary_expression -> expression bin_op expression;

bin_op -> "PLUS"
       | "MINUS"
       | "LTEQ"
       | "GTEQ"
       | "LT"
       | "GT"
       | "EQEQ"
       ;

literal -> "INT_LITERAL"
        ;

%%
pub struct Prog(Vec<statement>);
use crate::config_ast::{Statement, Expr, BinOp};