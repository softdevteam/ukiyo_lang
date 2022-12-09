use lrpar::Span;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        span: Span,
        id: Span,
        expr: Box<Expr>,
    },
    Print {
        span: Span,
        args: Box<Expr>,
    },
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
    While {
        span: Span,
        condition: Box<Expr>,
        body: Box<Expr>,
    },
}
impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Assign { span, .. } => *span,
            Expr::String(span) => *span,
            Expr::BinaryOp { span, .. } => *span,
            Expr::Int { span, .. } => *span,
            Expr::VarLookup(span) => *span,
            Expr::Print { span, .. } => *span,
            Expr::While { span, .. } => *span,
        }
    }
}