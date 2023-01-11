use lrpar::Span;

#[derive(Debug, Clone)]
pub enum Expr {
    ExprList(Vec<Expr>),
    Prog {
        span: Span,
        stmts: Vec<Expr>,
    },
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
    WhileLoop {
        span: Span,
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    IfStatement {
        span: Span,
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    FuncDef {
        span: Span,
        name: Span,
        args_list: Vec<Span>,
        body: Box<Expr>,
    },
    Call {
        span: Span,
        name: Span,
        params: Box<Expr>,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::ExprList(v) => v[0].span(),
            Expr::Assign { span, .. } => *span,
            Expr::String(span) => *span,
            Expr::BinaryOp { span, .. } => *span,
            Expr::Int { span, .. } => *span,
            Expr::VarLookup(span) => *span,
            Expr::Print { span, .. } => *span,
            Expr::WhileLoop { span, .. } => *span,
            Expr::IfStatement { span, .. } => *span,
            Expr::Prog { span, .. } => *span,
            Expr::FuncDef { span, .. } => *span,
            Expr::Call { span, .. } => *span,
        }
    }
}