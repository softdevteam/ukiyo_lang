use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
pub mod compiler;
pub mod config_ast;
pub mod vm;
use vm::run;
lrlex_mod!("lib/ukiyo.l");
lrpar_mod!("lib/ukiyo.y");

pub fn compile(contents: String) {
    // Use the contents string as needed within the function
    let lexerdef = ukiyo_l::lexerdef();
    let lexer = lexerdef.lexer(&contents);
    let (res, errs) = ukiyo_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &ukiyo_y::token_epp));
    }
    match res {
        Some(Ok(r)) => {
            run(r, &lexer);
        }
        _ => eprintln!("Unable to evaluate expression."),
    }
}
