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
    let lines: Vec<&str> = contents.lines().collect();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let lexer = lexerdef.lexer(line);
        let (res, errs) = ukiyo_y::parse(&lexer);
        for e in errs {
            println!("{}", e.pp(&lexer, &ukiyo_y::token_epp));
        }
        match res {
            Some(Ok(r)) => {
                println!("{:?}", run(r, &lexer));
            }
            _ => eprintln!("Unable to evaluate expression."),
        }
    }
}
