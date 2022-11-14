%start prog
%%
prog -> Result<(), Box<dyn Error>>: 
            prog "SEMICOLON" statement { Ok(()) }
          | statement { Ok(()) }
          ;

statement -> Result<(), Box<dyn Error>>: 
            expression { Ok(()) }
          | assigment { Ok(()) }
          ;

assigment -> Result<(), Box<dyn Error>>: 
          "LET" "IDENTIFIER" "EQ" expression { Ok(()) };

expression -> Result<(), Box<dyn Error>>:
            variable {Ok(()) }
          // | binary_expression { Ok() }
          | literal { Ok(()) }
          ;

variable -> Result<(), Box<dyn Error>>: 
           "IDENTIFIER" { Ok(()) };

binary_expression -> Result<(), Box<dyn Error>>: 
           expression bin_op expression { Ok(()) };

bin_op -> Result<(), Box<dyn Error>>: 
          "PLUS"  { Ok(()) }
        | "MINUS" { Ok(()) }
        | "LTEQ"  { Ok(()) }
        | "GTEQ"  { Ok(()) }
        | "LT"    { Ok(()) }
        | "GT"    { Ok(()) }
        | "EQEQ"  { Ok(()) }
        ;

literal -> Result<(), Box<dyn Error>>: 
          "INT" { Ok (()) };
%%

pub struct Prog(Vec<Statement>);

//use lrpar::{Span};
use crate::config_ast::{Statement};
use std::error::Error;