use crate::compiler::{ compiler, OpCode, Ast };
use lrlex::{ DefaultLexeme };
use lrpar::NonStreamingLexer;
use std::{ fmt, collections::HashMap };

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
                //println!("in pushint");
                stack.push(Types::Int(x.clone()));
            }
            OpCode::PushStr(..) => {
                //stack.push(Types::String(x.clone()));
                todo!();
            }
            OpCode::StoreVar(ref idx) => {
                //println!("in storeint");
                let val = stack.pop().unwrap();
                let len = locals.len();
                if *idx < len {
                    locals[*idx] = val.clone();
                } else {
                    locals.push(val);
                }
            }
            OpCode::LoadVar(ref idx) => {
                //println!("we now in loadvar");
                let val = locals[*idx].clone();
                stack.push(val);
            }
            OpCode::Call(label) => {
                //println!("in call");
                if label == "print" {
                    let mut output = String::new();

                    if let Some(val) = stack.pop() {
                        let val_copy = val.to_owned();
                        match val {
                            Types::Int(x) => output.push_str(&x.to_string()),
                            Types::Bool(x) => output.push_str(&x.to_string()),
                            Types::String(x) => output.push_str(&x),
                            Types::NoneType => output.push_str("None"),
                        }
                        stack.push(val_copy);
                    } else {
                        // Return an error if the stack is empty
                        return Err("Cannot call print on empty stack".to_string());
                    }

                    println!("{}", output);
                }
            }
            OpCode::Plus => {
                //println!("in plus");
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x + y)),
                    _ => panic!("TypeError"),
                }
            }
            OpCode::Minus => {
                //println!("in minus");
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x - y)),
                    _ => panic!("TypeError"),
                }
            }
            OpCode::Lteq => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    if let (Types::Int(lhs_val), Types::Int(rhs_val)) = (lhs, rhs) {
                        stack.push(Types::Bool(lhs_val <= rhs_val));
                    } else {
                        return Err("Cannot compare values of different types".to_string());
                    }
                } else {
                    return Err("Cannot compare values on empty stack".to_string());
                }
            }

            OpCode::Lt => {
                println!("in LT");
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    if let (Types::Int(lhs_val), Types::Int(rhs_val)) = (lhs, rhs) {
                        let res = Types::Bool(lhs_val < rhs_val);
                        println!("is true or false?: {}", res);
                        stack.push(Types::Bool(lhs_val < rhs_val));
                    } else {
                        return Err("Cannot compare values of different types".to_string());
                    }
                } else {
                    return Err("Cannot compare values on empty stack".to_string());
                }
            }

            OpCode::Jump(pos) => {
                //println!("in jump");
                // Loop through the program and find the instruction with the specified label
                pc = *pos;
            }
            OpCode::JumpIfFalse(pos) => {
                //println!("in jump if false");
                let val = stack.pop().unwrap();
                if let Types::Bool(false) = val {
                    pc = *pos;
                }
            }
            OpCode::Label(label) => {
                //println!("in label()");
                // Labels do not have any effect on the execution of the program,
                // so they can be ignored when they are encountered
            }
        }
        pc += 1;
    }
    //let l = stack.len();
    //println!("length of stack is: {l}");
    //assert_eq!(stack.len(), 1);
    println!("stack length is: {}", stack.len());
    Ok(stack)
}

pub fn run(
    ast: Ast,
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>
) -> Result<Vec<Types>, String> {
    let prog = compiler(ast, lexer)?;
    let res = vm(prog)?;
    Ok(res)
}

// #[cfg(test)]
// mod test {
//     use lrlex::lrlex_mod;
//     use lrpar::lrpar_mod;
//     lrlex_mod!("lib/ukiyo.l");
//     lrpar_mod!("lib/ukiyo.y");

//     use crate::vm::run;

//     fn compile_and_run(input: &str) -> String {
//         let lexerdef = ukiyo_l::lexerdef();
//         let lexer = lexerdef.lexer(input);
//         let res = ukiyo_y::parse(&lexer).0.unwrap().unwrap();
//         let output = run(res, &lexer);
//         let mut res_str = String::new();
//         for element in output.iter() {
//             res_str.push_str(&format!("[{}] ", element));
//         }
//         res_str.trim_end().to_string()
//     }
//     #[test]
//     // fn basic() {
//     //     assert_eq!(compile_and_run("2+3;"), "[5]");
//     //     assert_eq!(compile_and_run("2+3+4;"), "[9]");
//     //     assert_eq!(compile_and_run("2 + -3;"), "[-1]");
//     //     assert_eq!(compile_and_run("2 - 3"), "[-1]");
//     //     assert_eq!(compile_and_run("2 <= 3"), "[true]");
//     // }
//     // #[test]
//     // fn print_test() {
//     //     assert_eq!(compile_and_run("let a = 1; let a = 2; let b = a + 3; print(b);"), "[5]");
//     //     assert_eq!(compile_and_run("let a = 4; let b = 2; let a = b + 3; print(a);"), "[5]");
//     //     assert_eq!(
//     //         compile_and_run("let a = 1; let b = 2; let a = b + 3; print(a); print(b);"),
//     //         "[5] [2]"
//     //     );
//     //     assert_eq!(
//     //         compile_and_run(
//     //             "let a = 1; let a = 2; let b = a + 3; let z = a + b; print(z); print(b);"
//     //         ),
//     //         "[7] [5]"
//     //     );
//     // }
//     fn while_loop_test() {
//         assert_eq!(
//             compile_and_run(
//                 "let a = 1;
//             let b = 3;
//             while (a <= b) {
//                 print(a);
//                 a = a + 1;
//             }"
//             ),
//             "[1] [2] [3]"
//         )
//     }
// }