%start prog
%%
prog -> Result<Vec<Expr>, ()>: 
            prog statement { flattenr($1, $2) }
          | statement { Ok(vec![$1?]) }
          ;

statement -> Result<Expr, ()>: 
            binary_expression "SEMICOLON" { $1 }
          | assigment  "SEMICOLON" { $1 }
          | print_statement "SEMICOLON" { $1 }
          | while_loop { $1 }
          | if_statement { $1 }
          | func_def { $1 }
          ;

func_def -> Result<Expr, ()>:
            "FUNC" "IDENTIFIER" "LBRACK" args_list "RBRACK" body {
            Ok(Expr::FuncDef {  span: $span, name: map_err($2)?, args_list: $4?,
                body: Box::new($6?),}) 
            };

args_list -> Result<Vec<Span>, ()>:
            { Ok(vec![]) }
          | args { $1 }
          ;

args -> Result<Vec<Span>,  ()>:
          "IDENTIFIER" { Ok(vec![map_err($1)?]) }
        | args "COMMA" "IDENTIFIER" { flattenr_span($1, $3) }
        ;
        
if_statement -> Result<Expr, ()>:
                "IF" binary_expression body {
                  Ok(Expr::IfStatement { span: $span, condition: Box::new($2?), body: Box::new($3?)})
                };
print_statement -> Result<Expr, ()>: 
                   "PRINT" "LBRACK" binary_expression "RBRACK" {  
                   Ok(Expr::Print { span: $span, args: Box::new($3?)})
                 };

while_loop -> Result<Expr, ()>:
              "WHILE" "LBRACK" binary_expression "RBRACK"  body {
              Ok(Expr::WhileLoop { span: $span, condition: Box::new($3?), body: Box::new($5?)})
              };
              


body -> Result<Expr, ()>:
        "LBRACE" prog "RBRACE" { Ok(Expr::Prog { span: $span, stmts: $2?}) }
        ;
          

assigment -> Result<Expr, ()>: 
          "LET" "IDENTIFIER" "EQ" binary_expression {  
            Ok(Expr::Assign { span: $span, id: map_err($2)?, expr: Box::new($4?)})
            };

unit -> Result<Expr, ()>:
        literal { $1 }
      | "IDENTIFIER" { Ok(Expr::VarLookup(map_err($1)?)) } 
      | "LBRACK" binary_expression "RBRACK" { $2 }
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

binary_term -> Result<Expr, ()>:
               unit { $1 }
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
use crate::config_ast::{ Expr };
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
/// Flatten `rhs` into `lhs`.
fn flattenr_span(lhs: Result<Vec<Span>, ()>, rhs: Result< DefaultLexeme<StorageT>, DefaultLexeme<StorageT>>) -> Result<Vec<Span>, ()> {
    let mut flt = lhs?;
    flt.push(rhs.map_err(|_| ())?.span());
    Ok(flt)
}