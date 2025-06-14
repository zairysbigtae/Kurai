// use kurai_codegen::codegen::codegen::CodeGen;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::value::Value;
use kurai_stmt::stmt::Stmt;
use kurai_expr::expr::Expr;
use kurai_binop::bin_op::BinOp;
use kurai_token::token::token::Token;
use kurai_token::eat::eat;

use crate::parse::parse_stmt::parse_stmt;
use crate::{FunctionParser, ImportParser};

// this function just wants to return stmt
// this function practically just runs whatever function here whenever the program encounters
// one of the tokens
pub fn parse_expr(tokens: &[Token], pos: &mut usize, in_condition: bool) -> Option<Expr> {
    // parse_equal(tokens, pos)
    let mut left = match tokens.get(*pos)? {
        Token::Number(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Int(*v)))
        }
        Token::StringLiteral(v) => {
            *pos += 1;
            let v = v.clone();
            Some(Expr::Literal(Value::Str(v)))
        }
        Token::Bool(v) => {
            *pos += 1;
            Some(Expr::Literal(Value::Bool(*v)))
        }
        Token::Id(id) => {
            let name = id.clone();
            *pos += 1;

            if eat(&Token::OpenParenthese, tokens, pos) {
                let mut args = Vec::new();
                while !eat(&Token::CloseParenthese, tokens, pos) {
                    if let Some(arg) = parse_expr(tokens, pos, false) {
                        args.push(arg);
                        eat(&Token::Comma, tokens, pos);
                    } else {
                        return None;
                    }
                }
                Some(Expr::FnCall { 
                    name,
                    args
                })
            } else {
                Some(Expr::Var(name))
            }
        }
        Token::OpenParenthese => {
            *pos += 1;
            let expr = parse_expr(tokens, pos, false)?;
            eat(&Token::CloseParenthese, tokens, pos);
            Some(expr)
        }
        _ => {
            *pos += 1;
            None
        }
    }?;

    // while let Some(Token::EqualEqual) = tokens.get(*pos) {
    //     *pos += 1;
    //     let right = parse_expr(tokens, pos)?;
    //
    //     left = Expr::Binary { 
    //         op: BinOp::Eq,
    //         left: Box::new(left),
    //         right: Box::new(right)
    //     };
    // }
    if in_condition {
        while let Some(token) = tokens.get(*pos) {
            let op = match token {
                Token::Less => BinOp::Lt,
                Token::LessEqual => BinOp::Le,
                Token::EqualEqual => BinOp::Eq,
                Token::Greater => BinOp::Gt,
                Token::GreaterEqual => BinOp::Ge,
                _ => break,
            };

            *pos += 1;
            let right = parse_expr(tokens, pos, in_condition)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right)
            }
        }
    }
    Some(left)
}

pub fn parse_typed_arg(tokens: &[Token], pos: &mut usize) -> Option<TypedArg> {
    todo!()
}

pub fn parse_out_vec_expr(tokens: &[Token]) -> Result<Vec<Expr>, String> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        if let Some(expr) = parse_expr(tokens, &mut pos, false) {
            exprs.push(expr);
            if eat(&Token::Comma, tokens, &mut pos) { continue; }
        }
    }

    Ok(exprs)
}

pub fn parse_out_vec_stmt(
    tokens: &[Token],
    discovered_modules: &mut Vec<String>,
    fn_parser: &dyn FunctionParser,
    import_parser: &dyn ImportParser,
) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();

    while pos < tokens.len() {
        match parse_stmt(tokens, &mut pos, discovered_modules, fn_parser, import_parser) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => panic!("Parse error at token {:?}: {}\n {:?}", tokens.get(pos), e, tokens)
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("TOKENS: {:?}", tokens);
    }
    stmts
}
