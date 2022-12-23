use crate::compiler::{ compiler, Ast, OpCode };
use core::panic;
use lrlex::DefaultLexeme;
use lrpar::NonStreamingLexer;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Types {
    Int(i32),
    String(String),
    Bool(bool),
    NoneType,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub prog: Vec<OpCode>,
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
    //initialize with exact size for locals.
    let mut locals: Vec<Types> = Vec::new();
    let mut functions: Vec<Function> = Vec::new();
    while pc < prog.len() {
        let expr = &prog[pc];
        match &*expr {
            OpCode::PushInt(ref x) => {
                stack.push(Types::Int(*x));
                pc += 1;
            }
            OpCode::PushStr(ref x) => {
                stack.push(Types::String(x.clone()));
                pc += 1;
            }
            //how to optimize this function?
            OpCode::StoreVar(ref idx) => {
                //change here to handle default cases foe each type
                let val = stack.pop().unwrap_or_else(|| Types::Int(0));
                let len = locals.len();
                if *idx < len {
                    locals[*idx] = val;
                } else {
                    locals.push(val);
                }
                pc += 1;
            }
            OpCode::LoadVar(ref idx) => {
                let val = locals[*idx].clone();
                stack.push(val);
                pc += 1;
            }
            OpCode::Call(label) => {
                // check if the function is a built-in function
                if label == "print" {
                    // execute the built-in function
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
                    }

                    println!("{}", output);
                } else {
                    // search for the user-defined function
                    println!("in user define func");
                    let func = functions.iter().find(|f| f.name == *label);
                    if let Some(func) = func {
                        // execute the user-defined function
                        let res = vm(func.prog.clone())?;
                        stack.extend(res);
                    } else {
                        return Err(format!("Function '{}' not found", label));
                    }
                }
                pc += 1;
            }
            OpCode::Plus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x + y)),
                    _ => panic!("TypeError"),
                }
                pc += 1;
            }
            OpCode::Minus => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match (lhs, rhs) {
                    (Types::Int(x), Types::Int(y)) => stack.push(Types::Int(x - y)),
                    _ => panic!("TypeError"),
                }
                pc += 1;
            }
            OpCode::Eqeq => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    if let (Types::Int(lhs_val), Types::Int(rhs_val)) = (lhs, rhs) {
                        stack.push(Types::Bool(lhs_val == rhs_val));
                    } else {
                        return Err("Cannot compare values of different types".to_string());
                    }
                } else {
                    return Err("Cannot compare values on empty stack".to_string());
                }
                pc += 1;
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
                pc += 1;
            }

            OpCode::Lt => {
                if let (Some(ref rhs), Some(ref lhs)) = (stack.pop(), stack.pop()) {
                    if let (Types::Int(lhs_val), Types::Int(rhs_val)) = (lhs, rhs) {
                        stack.push(Types::Bool(lhs_val < rhs_val));
                    } else {
                        return Err("Cannot compare values of different types".to_string());
                    }
                } else {
                    return Err("Cannot compare values on empty stack".to_string());
                }
                pc += 1;
            }

            OpCode::Jump(pos) => {
                pc = *pos;
            }
            OpCode::JumpIfFalse(pos) => {
                let val = stack.pop().unwrap();

                if let Types::Bool(false) = val {
                    //panic!();
                    //assert!(*pos != usize::max_value());
                    pc = *pos;
                } else {
                    pc += 1;
                }
            }

            OpCode::Return => {
                return Ok(stack);
            }

            OpCode::DefineFunc(name, func_prog) => {
                // Define a new function in the VM
                let func = Function {
                    name: name.to_string(),
                    prog: func_prog.to_vec(),
                };
                functions.push(func);
                println!("functions is: {:?}", functions);
                pc += 1;
            }

            // OpCode::Call(name) => {
            //     let func = functions
            //         .iter()
            //         .find(|f| f.name == *name)
            //         .unwrap();
            //     let res = vm(func.prog.to_vec());
            //     stack.extend(res.unwrap());
            //     pc += 1;
            // }
        }
    }
    Ok(stack)
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<Types> {
    let prog = compiler(ast, lexer);
    dbg!(&prog);
    //panic!();
    let res = vm(prog.unwrap());
    res.unwrap()
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
//     fn str_test() {
//         assert_eq!(compile_and_run("\"\""), "[]");
//         assert_eq!(compile_and_run("\"hello\""), "[hello]");
//         assert_eq!(
//             compile_and_run("let a = \"Hello, World!\" ; let b = a; print(b);"),
//             "[Hello, World!]"
//         );
//         assert_eq!(
//             compile_and_run(
//                 "let a = \"Hello, World!\" ; let i = 0; while (i < 2) { print(a); let i = i + 1;}"
//             ),
//             "[Hello, World!] [Hello, World!]"
//         );
//     }
//     #[test]
//     fn basic() {
//         assert_eq!(compile_and_run("2+3;"), "[5]");
//         assert_eq!(compile_and_run("2+3+4;"), "[9]");
//         assert_eq!(compile_and_run("2 + -3;"), "[-1]");
//         assert_eq!(compile_and_run("2 - 3"), "[-1]");
//         assert_eq!(compile_and_run("2 <= 3"), "[true]");
//     }
//     #[test]
//     fn print_test() {
//         assert_eq!(compile_and_run("let a = 1; let a = 2; let b = a + 3; print(b);"), "[5]");
//         assert_eq!(compile_and_run("let a = 4; let b = 2; let a = b + 3; print(a);"), "[5]");
//         assert_eq!(
//             compile_and_run("let a = 1; let b = 2; let a = b + 3; print(a); print(b);"),
//             "[5] [2]"
//         );
//         assert_eq!(
//             compile_and_run(
//                 "let a = 1; let a = 2; let b = a + 3; let z = a + b; print(z); print(b);"
//             ),
//             "[7] [5]"
//         );
//     }
//     #[test]
//     fn if_statement() {
//         assert_eq!(compile_and_run("let a = 1; let b = 2; if(a < b) { print(a); }"), "[1]");
//     }
//     #[test]
//     fn while_loop_test() {
//         // assert_eq!(
//         //     compile_and_run(
//         //         "let z = 0; while (z < 1) {print(z); let z = z + 1; print(z); }
//         //         "
//         //     ),
//         //     "[0] [1]"
//         // );
//         // assert_eq!(
//         //     compile_and_run(
//         //         "let x = 0; let y = 5  while (x < y) {print(x); let x = x + 1; print(y); let y = y - x; }
//         //         "
//         //     ),
//         //     "[0] [5] [1] [4]"
//         // );

//         assert_eq!(
//             compile_and_run("let x = 2; let y = 1; while (x < y ) {print(1); }
//                 "),
//             ""
//         );
//         // assert_eq!(
//         //     compile_and_run("let x = 2; let y = 2; while (x == y) { print(0); let y = y + 1; }"),
//         //     "[0]"
//         // )
//     }
// }