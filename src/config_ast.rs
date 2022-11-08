use cfgrammar::Span;
pub enum Statement {
    Assign {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    }
    Expr,
}
pub enum Expr {
    Literal(i32),
    BinOp {
        span: Span,
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Var(String),
}
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    GtEq,
    LtEq,
}