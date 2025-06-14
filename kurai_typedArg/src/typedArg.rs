use kurai_expr::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct TypedArg {
    pub name: String,
    pub typ: String,
    pub value: Option<Expr>,
}
