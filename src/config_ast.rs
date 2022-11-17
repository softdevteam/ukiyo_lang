use lrpar::Span;

// #[derive(Debug)]
// pub enum Statement {
//     Assign: Vec<Expr>,
//     Expr,
// }
#[derive(Debug)]
pub enum Expr {
    Literal(Span),
    Assign {
        span: Span,
        id: Span,
        expr: Box<Expr>,
    },
    BinaryOp {
        span: Span,
        op: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BinaryTerm {
        span: Span,
        receiver: Box<Expr>,
        ids: Vec<Span>,
    },
    Int {
        span: Span,
        is_negative: bool,
        val: Span,
    },
    String(Span),
    VarLookup(Span),
}
impl Expr {
    pub fn span(&self) -> Span {
        match self {
           Expr::Assign { span, .. } => *span,
           Expr::Literal(span) => *span,
           Expr::String(span) => *span,
           Expr::BinaryOp { span, .. } => *span,
           Expr::BinaryTerm { span, .. } => *span,
           Expr::Int { span, .. } => *span,
           Expr::VarLookup(span) => *span,
        }
    }
}

// impl Statement {
//     pub fn span(&self) -> Span {
//         match self {
//             Statement::Assign{span, .. } => *span,
//             Statement::Expr => todo!(),
//         }
//     }
// }