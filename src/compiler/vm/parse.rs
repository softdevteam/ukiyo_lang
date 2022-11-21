#[derive(Debug, Clone)]
pub enum Instr {
    PushInt(i32),
    PushStr(String),
    Pop,
    Add,
    Sub,
    Lteq,
    Gteq,
    Lt,
    Gt,
    Eqeq,
    LoadVar(usize),
    StoreVar(usize),
    LoadGlobal(String),
    StoreGlobal(String),
}