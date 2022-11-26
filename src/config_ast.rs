use lrpar::Span;

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
           Expr::Int { span, .. } => *span,
           Expr::VarLookup(span) => *span,
        }
    }
}
