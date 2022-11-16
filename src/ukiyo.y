%start prog
%%
prog -> Result<(), Box<dyn Error>>: 
            prog "SEMICOLON" statement { Ok(()) }
          | statement { Ok(()) }
          ;

statement -> Result<(), Box<dyn Error>>: 
            expression { $1 }
          | assigment { $1 }
          ;

expression -> Result<(), Box<dyn Error>>:
            unit { Ok(()) }
          | binary_expression { Ok(()) }
          ;

assigment -> Result<(), Box<dyn Error>>: 
          "LET" "IDENTIFIER" "EQ" expression { Ok(()) };

unit -> Result<(), Box<dyn Error>>:
        "IDENTIFIER" { $1 }
      | literal { $1 }
      | "LBRACK" expression "RBRACK" { $2 }
      ;
          
literal -> Result<(), Box<dyn Error>>: 
          "INT" { Ok(()) }
        | "MINUS" "INT" { Ok(()) }
        | "STRING" { Ok(()) }
        ;

binary_expression -> Result<(), Box<dyn Error>>: 
                    binary_expression bin_op binary_term { Ok(()) }
                  | binary_term { $1 }
                  ;

binary_term -> Result<(), Box<dyn Error>>:
               unit idlistOpt { Ok(()) };

idlistOpt -> Result<(), Box<dyn Error>>:
            idlist { $1 };

idlist -> Result<(), Box<dyn Error>>:
          "IDENTIFIER" { $1 }
        | idlist "IDENTIFIER" { Ok(()) }
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

use crate::config_ast::{Statement};
use std::error::Error;
// use lrlex::DefaultLexeme;
// use lrpar::Span;

// type StorageT = u32;

// fn map_err(r: Result<DefaultLexeme<StorageT>, DefaultLexeme<StorageT>>)
//     -> Result<Span, ()>
// {
//     r.map(|x| x.span()).map_err(|_| ())
// }