use crate::compiler::{ compiler, OpCode, Ast };
use lrlex::{ DefaultLexeme };
use lrpar::NonStreamingLexer;

fn vm(prog: Vec<OpCode>) -> Result<Vec<i32>, String> {
    if prog.is_empty() {
        return Err("Cannot execute empty program".to_string());
    }
    let mut pc = 0;
    let mut stack: Vec<i32> = Vec::new();
    let mut locals: Vec<i32> = Vec::new();
    while pc < prog.len() {
        let expr = &prog[pc];

        match *expr {
            OpCode::Int(x) => {
                stack.push(x);
            }
            OpCode::Str(..) => {
                // stack.push(OpCode::Str(val.clone()));
                // pc += 1;
                todo!();
            }
            OpCode::StoreVar(idx) => {
                let val = stack.pop().unwrap_or(0);
                locals.insert(idx, val);
                stack.push(val);
            }
            OpCode::LoadVar(idx) => {
                let val = locals[idx];
                stack.push(val);
            }
            OpCode::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            OpCode::Minus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            OpCode::Lteq => {
                // let rhs = stack.pop().unwrap();
                // let lhs = stack.pop().unwrap();
                // stack.push(lhs < rhs);
                todo!();
            }
        }
        pc += 1;
    }
    //let l = stack.len();
    //println!("length of stack is: {l}");
    //assert_eq!(stack.len(), 1);
    Ok(stack)
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<i32> {
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
    }
    #[test]
    fn test2() {
        assert_eq!(compile_and_run("let x = 1+2; let y = x+1; let z = x + y;"), "[3] [4] [7]");
    }
}