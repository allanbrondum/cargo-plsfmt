use crate::replacement::Replacement;
use proc_macro2::Ident;
use syn::parse::discouraged::AnyDelimiter;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Macro, Pat, Token, braced, token};

pub fn select_replace(mac: &Macro) -> Replacement {
    todo!()
}

struct Select {
    arms: Vec<SelectArm>,
}

struct SelectArm {
    pat: Pat,
    eq: Token![=],
    future_expr: Box<Expr>,
    fat_arrow: Token![=>],
    // brace_token: token::Brace,
    expr: Box<Expr>,
}

impl Parse for SelectArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // let content;
        let arm = SelectArm {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            eq: input.parse()?,
            future_expr: input.parse()?,
            fat_arrow: input.parse()?,
            // brace_token: braced!(content in input),
            // expr: content.parse()?,
            expr: input.parse()?,
        };
        // if !content.is_empty() {
        //     content.error("left over content");
        // }

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
}
