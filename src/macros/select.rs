use crate::replacement::Replacement;
use syn::parse::discouraged::AnyDelimiter;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Expr, Macro, Pat, Token};

pub fn select_replace(mac: &Macro) -> Replacement {
    todo!()
}

struct Select {
    arms: Vec<Arm>,
}

struct Arm {
    pat: Pat,
    eq: Token![=],
    future: Expr,
    fat_arrow: Token![=>],
    body: Expr,
    comma: Option<Comma>,
}

impl Parse for Arm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let requires_comma;
        let arm = Arm {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            eq: input.parse()?,
            future: input.parse()?,
            fat_arrow: input.parse()?,
            body: {
                let body = Expr::parse_with_earlier_boundary_rule(input)?;
                requires_comma = requires_comma_to_be_match_arm(&body)?;
                body
            },
            comma: {
                if requires_comma && !input.is_empty() {
                    Some(input.parse()?)
                } else {
                    input.parse()?
                }
            },
        };

        Ok(arm)
    }
}

impl Parse for Select {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(input.parse()?);
        }

        Ok(Self { arms })
    }
}

fn requires_comma_to_be_match_arm(expr: &Expr) -> syn::Result<bool> {
    match expr {
        Expr::If(_)
        | Expr::Match(_)
        | Expr::Block(_) | Expr::Unsafe(_) // both under ExprKind::Block in rustc
        | Expr::While(_)
        | Expr::Loop(_)
        | Expr::ForLoop(_)
        | Expr::TryBlock(_)
        | Expr::Const(_) => Ok(false),

        Expr::Array(_)
        | Expr::Assign(_)
        | Expr::Async(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Break(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Closure(_)
        | Expr::Continue(_)
        | Expr::Field(_)
        | Expr::Group(_)
        | Expr::Index(_)
        | Expr::Infer(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Macro(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Range(_)
        | Expr::RawAddr(_)
        | Expr::Reference(_)
        | Expr::Repeat(_)
        | Expr::Return(_)
        | Expr::Struct(_)
        | Expr::Try(_)
        | Expr::Tuple(_)
        | Expr::Unary(_)
        | Expr::Yield(_)
        | Expr::Verbatim(_) => Ok(true),
        _ => Err(syn::Error::new(expr.span(), format!("unhandled expression"))),
    }
}

#[cfg(test)]
mod test {
    use crate::macros::select::Select;
    use syn::Macro;
    use syn::parse::{Parse, Parser};

    #[test]
    fn test_parse_select_braced_expr() {
        let code = r#"
select! {
    val1 = future1.expr() => {
        a.expr()
    }
    val2 = future2.expr() => {
        b.expr()
    }
}
        "#;

        let mac: Macro = syn::parse_str(code).unwrap();
        let select = Parser::parse2(Select::parse, mac.tokens).unwrap();
    }

    #[test]
    fn test_parse_select_unbraced_expr() {
        let code = r#"
select! {
    val1 = future1.expr() => a.expr(),
    val2 = future2.expr() => b.expr(),
    val3 = future3.expr() => {
        c.expr()
    }
    val4 = future4.expr() => d.expr()
}
        "#;

        let mac: Macro = syn::parse_str(code).unwrap();
        let select = Parser::parse2(Select::parse, mac.tokens).unwrap();
    }

    #[test]
    fn test_parse_select_unused_future() {
        let code = r#"
select! {
    _ = future1.expr() => {
        a.expr()
    }
}
        "#;

        let mac: Macro = syn::parse_str(code).unwrap();
        let select = Parser::parse2(Select::parse, mac.tokens).unwrap();
    }

    #[test]
    fn test_parse_select_let_pattern() {
        let code = r#"
select! {
    Some(val1) = future1.expr() => {
        a.expr()
    }
}
        "#;

        let mac: Macro = syn::parse_str(code).unwrap();
        let select = Parser::parse2(Select::parse, mac.tokens).unwrap();
    }
}
