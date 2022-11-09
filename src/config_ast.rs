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
    Number {
        span: Span
    },
    Int {
        span: Span,
        is_negative: bool,
        val: Span,
    },
    Var(String),
    VarLookUp(Span),
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