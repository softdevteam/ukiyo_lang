use crate::compiler::{ compiler, OpCode, Ast };
use lrlex::{ DefaultLexeme };
use lrpar::NonStreamingLexer;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Types {
    Int(i32),
    String(String),
    Bool(bool),
    NoneType,
}

impl Types {
    fn pretty(&self) -> String {
        match *self {
            Types::Int(ref x) => x.to_string(),
            Types::Bool(ref x) => x.to_string(),
            Types::String(ref x) => x.to_string(),
            Types::NoneType => "None".to_string(),
        }
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

fn vm(prog: Vec<OpCode>) -> Result<Vec<Types>, String> {
    if prog.is_empty() {
        return Err("Cannot execute empty program".to_string());
    }
    let mut pc = 0;
    let mut stack: Vec<Types> = Vec::new();
    let mut locals: Vec<Types> = Vec::new();
    while pc < prog.len() {
        let expr = &prog[pc];

        match &*expr {
            OpCode::PushInt(ref x) => {
                stack.push(Types::Int(x.clone()));
            }
            OpCode::PushStr(ref x) => {
                stack.push(Types::String(x.clone()));
                // pc += 1;
                todo!();
            }
            OpCode::StoreVar(idx) => {
                let val = stack.pop().unwrap();
                let len = locals.len();
                if *idx < len {
                    locals[*idx] = val.clone();
                } else {
                    locals.push(val);
                }
            }
            OpCode::LoadVar(idx) => {
                println!("we now in loadvar");
                let val = locals[*idx].clone();
                stack.push(val);
            }
            OpCode::Call(label) => {
                if label == "print" {
                    let mut output = String::new();

                    let val = stack.pop().unwrap();
                    let val_copy = val.to_owned();
                    match val {
                        Types::Int(x) => output.push_str(&x.to_string()),
                        Types::Bool(x) => output.push_str(&x.to_string()),
                        Types::String(x) => output.push_str(&x),
                        Types::NoneType => output.push_str("None"),
                    }
                    stack.push(val_copy);

                    println!("{}", output);
                }
            }
            OpCode::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x + y)),
                    _ => panic!("TypeError"),
                }
            }
            OpCode::Minus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x - y)),
                    _ => panic!("TypeError"),
                }
            }
            OpCode::Lteq => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Bool(x <= y)),
                    _ => panic!("TypeError"),
                }
            }
            OpCode::Lt => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Bool(x < y)),
                    _ => panic!("TypeError"),
                }
            }
        }
        pc += 1;
    }
    //let l = stack.len();
    //println!("length of stack is: {l}");
    //assert_eq!(stack.len(), 1);
    Ok(stack)
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<Types> {
    let prog = compiler(ast, lexer).unwrap();
    let res = vm(prog).unwrap();
    res
}

#[cfg(test)]
mod test {
    use lrlex::lrlex_mod;
    use lrpar::lrpar_mod;
    lrlex_mod!("lib/ukiyo.l");
    lrpar_mod!("lib/ukiyo.y");

    use crate::vm::run;

    fn compile_and_run(input: &str) -> String {
        let lexerdef = ukiyo_l::lexerdef();
        let lexer = lexerdef.lexer(input);
        let res = ukiyo_y::parse(&lexer).0.unwrap().unwrap();
        let output = run(res, &lexer);
        let mut res_str = String::new();
        for element in output.iter() {
            res_str.push_str(&format!("[{}] ", element));
        }
        res_str.trim_end().to_string()
    }
    #[test]
    fn basic() {
        assert_eq!(compile_and_run("2+3;"), "[5]");
        assert_eq!(compile_and_run("2+3+4;"), "[9]");
        assert_eq!(compile_and_run("2 + -3;"), "[-1]");
        assert_eq!(compile_and_run("2 - 3"), "[-1]");
        assert_eq!(compile_and_run("2 <= 3"), "[true]");
    }
    #[test]
    fn print_test() {
        assert_eq!(compile_and_run("let a = 1; let a = 2; let b = a + 3; print(b);"), "[5]");
        assert_eq!(compile_and_run("let a = 4; let b = 2; let a = b + 3; print(a);"), "[5]");
        assert_eq!(
            compile_and_run("let a = 1; let b = 2; let a = b + 3; print(a); print(b);"),
            "[5] [2]"
        );
    }
}