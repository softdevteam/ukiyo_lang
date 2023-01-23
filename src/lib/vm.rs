use crate::compiler::{compiler, Ast, OpCode};
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
    pub locals: Option<Vec<Types>>,
    pub args: Vec<String>,
    pub prog: Vec<OpCode>,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, prog: Vec<OpCode>) -> Self {
        Self {
            name,
            locals: None,
            args,
            prog,
        }
    }
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

fn vm(
    prog: Vec<OpCode>,
    locals: &mut Vec<Types>,
    functions: &mut Vec<Function>,
) -> Result<Vec<Types>, String> {
    if prog.is_empty() {
        return Err("Cannot execute empty program".to_string());
    }
    let mut pc = 0;
    let mut stack: Vec<Types> = Vec::new();

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
                    let func = functions.iter().find(|f| f.name == *label);
                    if let Some(func) = func {
                        for arg in func.args.iter() {
                            let val = stack.pop().unwrap();
                            locals.push(val);
                        }
                        let result = vm(func.prog.clone(), locals, functions)?;
                        locals.extend(result.clone());
                        stack.extend(result);
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
                    pc = *pos;
                } else {
                    pc += 1;
                }
            }

            OpCode::Return => {
                return Ok(stack);
            }

            OpCode::DefineFunc(name, args, func_prog) => {
                // Define a new function in the VM
                let func = Function {
                    name: name.to_string(),
                    args: args.to_vec(),
                    prog: func_prog.to_vec(),
                    locals: None,
                };
                functions.push(func);
                pc += 1;
            }
            OpCode::Patch => {
                unreachable!("Unabled to patch back value");
            }
        }
    }
    Ok(stack)
}

pub fn run(ast: Ast, lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>) -> Vec<Types> {
    let prog = compiler(ast, lexer);
    let mut locals = Vec::new();
    let mut functions: Vec<Function> = Vec::new();
    let res = vm(prog.unwrap(), &mut locals, &mut functions);
    res.unwrap()
}
