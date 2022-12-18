use cfgrammar::yacc::YaccKind;
use lrlex::{self, CTLexerBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("lib/ukiyo.y")
                .unwrap()
                .visibility(lrpar::Visibility::Public)
        })
        .lexer_in_src_dir("lib/ukiyo.l")?
        .visibility(lrlex::Visibility::Public)
        .build()?;
    Ok(())
}
