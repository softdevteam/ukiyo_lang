use crate::{Compile, Node, Operator};

#[derive(Debug, Clone)]
pub enum Instr {
    PushInt(i32),
    PushStr(String),
    Pop,
    Add,
    LoadVar(usize),
    StoreVar(usize),
}

#[derive(Debug)]
pub struct Bytecode {
    pub bytecode: Vec<Instr>,
}

impl Bytecode {
    fn new(ctx : CompilerContext) -> Bytecode {
        Bytecode {
            bytecode: ctx.bytecode,
        }
    }
}

struct CompilerContext<'pt> {
    bytecode: Vec<Instr>,

    // Fields for convenience when building up the Bytecode struct
    grm:        &'pt YaccGrammar,
    input:      &'pt str,
}

impl<'pt> CompilerContext<'pt> {
    fn new(grm: &'pt YaccGrammar, input: &'pt str) -> CompilerContext<'pt> {
        CompilerContext {
            constants: Vec::new(),
            bytecode: Vec::new(),
            grm:     grm,
            input:   input,
        }
    }

    fn gen_bc(&mut self , instr: Instr) -> usize {
        self.bytecode.push(instr);
        self.bytecode.len() - 1
    }

    fn get_value(&self, node: &Node<u16>) -> String {
        match *node {
            Node::Term { lexeme } => self.input[lexeme.start()..lexeme.start() + lexeme.len()]
                                        .to_string(),
            _ => panic!("Cannot determine name of non-terminal node")
        }
    }

    fn get_name(&self, node: &Node<u16>) -> String {
        match *node {
            Node::Nonterm { nonterm_idx, .. } => {
                self.grm.nonterm_name(nonterm_idx).to_string()
            }
            Node::Term { lexeme } => {
                let token_id: usize = lexeme.tok_id().try_into().ok().unwrap();
                self.grm.term_name(TIdx::from(token_id)).unwrap().to_string()
            }
        }
    }
}

fn gen_bytecode(parse_tree: &Node<u16>, grm: &YaccGrammar, input: &str) -> Bytecode {

    // prog : prog statement
    //      | statment 
    //      ;
    fn gen_prog(node: &Node<u16>, ctx: &mut CompilerContext) {
        if let &Node::Nonterm{ ref nodes, .. } = node {
            for child in nodes {
                match ctx.get_name(child).as_ref(){
                    "statement" => gen_stmt(child, ctx),
                    "prog" => gen_prog(child, ctx)
                }
            }
        }
    }

    // statement : binary_expression
    //           | assignment
    //           ;
    
    fn gen_stmt(node: &Node<u16>, ctx: &mut CompilerContext) {
        if let &Node::Nonterm{ ref nodes, .. } = node {
            match ctx.get_name(&nodes[0]).as_ref(){
                "expression"    => gen_exp(&nodes[0], ctx),
                "assignment"  => gen_assign(&nodes[0], ctx),
                _ => panic!("unknown nonterminal node")
            }
        }
    }

    fn gen_exp(node: &Node<u16>, ctx: &mut CompilerContext) {
        if let &Node::Nonterm{ ref nodes, .. } = node {
            let exp_type = &nodes[0];
            let name = ctx.get_name(exp_type);
            if let &Node::Nonterm{ ref nodes, .. } = exp_type {
                match name.as_ref() {
                    "binary_term" => {
                        gen_binterm(&nodes[0], ctx);
                    }
                    "binary_expression" => {
                        gen_exp(&nodes[0], ctx);
                        gen_exp(&nodes[2], ctx);
                        ctx.gen_bc(Instr::Add)
                    }
                    _ => panic!("unknown expression")
                }
            }
        }
    }

    fn gen_benterm(node: &Node<u16>, ctx: &mut CompilerContext) {
        match *node {
            
        }
    }
    //let_statement : "LET" "IDENTIFIER" "EQ" expression;
    fn gen_let(node: &Node<u16>, ctx: &mut CompilerContext) {
        if let &Node::Nonterm{ ref nodes, .. } = node {
            gen_exp(&nodes[3], ctx);
            ctx.gen_bc(Instr::StoreVar(&nodes[1]));
        }
    }
 
    let mut ctx = CompilerContext::new(grm, input);
    match *parse_tree {
        Node::Nonterm { ref nodes, .. } => {
            for cls in nodes.iter() {
                gen_prog(cls, &mut ctx);
            }
        }
        _ => panic!("Error")
    }
    Bytecode::new(ctx)
}

// #[cfg(test)]
// mod tests {
//     use std::path::Path;
//     use parse::{parse_input, Bytecode};
//     const LEX_PATH: &str = "src/ukiyo.l";
//     const YACC_PATH: &str = "src/ukiyo.y";

//     fn build_bytecode(source: String) -> Bytecode {
//         let lex_path = Path::new(LEX_PATH);
//         let yacc_path = Path::new(YACC_PATH);
//         parse_input(source, &lex_path, &yacc_path).unwrap()
//     }
//     // tests go here
// }

