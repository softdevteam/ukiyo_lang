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
            unit {Ok(()) }
          | binary_expression { Ok() }
          | literal { Ok(()) }
          ;
unit -> Result<(), Box<dyn Error>>:
        "IDENTIFIER" { $1 }
      | literal { $1 }
      | "LBRACK" expression "RBRACK" { $2 }
      ;
          
literal -> Result<(), Box<dyn Error>>: 
          "INT" { Ok (()) }
        | "-" "INT"
        | "STRING" { Ok(()) }
        ;

binary_expression -> Result<(), Box<dyn Error>>: 
                    expression bin_op binary_term { Ok(()) }
                  | binary_term { $1 }
                  ;

binary_term -> Result<(), Box<dyn Error>>:
               unit idlistOpt { Ok(()) };

idlistOpt -> Result<(), Box<dyn Error>>:
            Idlist { $1 }
          | { Ok(()) }
          ;

Idlist -> Result<(), Box<dyn Error>>:
          "IDENTIFIER" { $1 }
        | Idlist "IDENTIFIER" { Ok(()) }
        ;      

bin_op -> Result<(), Box<dyn Error>>: 
          "PLUS"  { Ok(()) }
        | "MINUS" { Ok(()) }
        | "LTEQ"  { Ok(()) }
        | "GTEQ"  { Ok(()) }
        | "LT"    { Ok(()) }
        | "GT"    { Ok(()) }
        | "EQEQ"  { Ok(()) }
        ;
%%

pub struct Prog(Vec<Statement>);

//use lrpar::{Span};
use crate::config_ast::{Statement};
use std::error::Error;