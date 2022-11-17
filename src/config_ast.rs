use lrpar::Span;

#[derive(Debug)]
pub enum Statement {
    Assign {
        span: Span,
        lhs: Box<Statement>,
        rhs: Box<Statement>,
    },
    Expr,
}
#[derive(Debug)]
pub enum Expr {
    Literal(Span),
    BinaryOp {
        span: Span,
        op: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Int {
        span: Span,
        is_negative: bool,
        val: Span,
    },
    String(Span),
    VarLookup(Span),
}
// #[derive(Debug)]
// pub enum BinOp {
//     PLUS,
//     MINUS,
//     LTEQ,
//     GTEQ,
//     LT,
//     GT,
//     EQEQ,
//     EQ,
//     COMMA,
// }

// impl Expr {
//     pub fn span(&self) -> Span {
//         match self {
//            Expr::Literal(span) => *span,
//            Expr::BinaryOp { span, .. } => *span,
//            Expr::Int { span, .. } => *span,
//            Expr::Var(span) => *span,
//            Expr::VarLookup(span) => *span,
//         }
//     }
// }

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Assign{span, .. } => *span,
            Statement::Expr => todo!(),
        }
    }
}