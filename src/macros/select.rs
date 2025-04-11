use crate::macro_syntax::{MacroFactory, MacroSyntax};
use prettyplease::algorithm::{BreakToken, Printer};
use prettyplease::fixup::FixupContext;
use prettyplease::iter::IterDelimited;
use prettyplease::{INDENT, expr, stmt};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Expr, Macro, Pat, Stmt, Token};

pub struct SelectFactory;

impl MacroFactory for SelectFactory {
    fn name() -> &'static str {
        "select"
    }

    fn parse(mac: &Macro) -> Option<Box<dyn MacroSyntax>> {
        Some(Box::new(
            Parser::parse2(SelectSyntax::parse, mac.tokens.clone()).ok()?,
        ))
    }
}

impl MacroSyntax for SelectSyntax {
    fn delimiter_replacement(&self, base_indent: isize) -> String {
        let mut printer = prettyplease::algorithm::Printer::new();
        // let base_indent = mac.path.span().start().column as isize;
        select(&mut printer, self, base_indent);
        printer.eof()
    }
}

struct SelectSyntax {
    arms: Vec<ArmSyntax>,
}

struct ArmSyntax {
    pat: Pat,
    #[allow(unused)]
    eq: Token![=],
    future: Expr,
    #[allow(unused)]
    fat_arrow: Token![=>],
    body: Expr,
    #[allow(unused)]
    comma: Option<Comma>,
}

impl Parse for ArmSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let requires_comma;
        let arm = ArmSyntax {
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

impl Parse for SelectSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(input.parse()?);
        }

        Ok(Self { arms })
    }
}

fn select(printer: &mut Printer, select_syntax: &SelectSyntax, base_indent: isize) {
    printer.word("{");
    printer.neverbreak();
    printer.cbox(INDENT + base_indent);
    printer.hardbreak_if_nonempty();
    for arm_syntax in &select_syntax.arms {
        arm(printer, arm_syntax);
        printer.hardbreak();
    }
    printer.offset(-INDENT);
    printer.end();
    printer.word("}");
}

fn arm(printer: &mut Printer, arm_syntax: &ArmSyntax) {
    printer.ibox(0);
    printer.pat(&arm_syntax.pat);
    printer.word(" = ");
    printer.expr(&arm_syntax.future, FixupContext::NONE);
    printer.word(" => ");

    let mut body = &arm_syntax.body;
    while let Expr::Block(expr) = body {
        if expr.attrs.is_empty() && expr.label.is_none() {
            let mut stmts = expr.block.stmts.iter();
            if let (Some(Stmt::Expr(inner, None)), None) = (stmts.next(), stmts.next()) {
                body = inner;
                continue;
            }
        }
        break;
    }

    if let Expr::Block(body) = body {
        if let Some(label) = &body.label {
            printer.label(label);
        }
        printer.word("{");
        printer.neverbreak();
        printer.cbox(INDENT);
        printer.hardbreak_if_nonempty();
        printer.inner_attrs(&body.attrs);
        for stmt in body.block.stmts.iter().delimited() {
            printer.stmt(&stmt, stmt.is_last);
        }
        printer.offset(-INDENT);
        printer.end();
        printer.word("}");
    } else {
        printer.neverbreak();
        printer.cbox(INDENT);
        let okay_to_brace = expr::parseable_as_stmt(body);
        printer.scan_break(BreakToken {
            pre_break: Some(if okay_to_brace { '{' } else { '(' }),
            ..BreakToken::default()
        });
        printer.expr_beginning_of_line(body, false, true, FixupContext::new_match_arm());
        printer.scan_break(BreakToken {
            offset: -INDENT,
            pre_break: (okay_to_brace && stmt::add_semi(body)).then_some(';'),
            post_break: if okay_to_brace { "}" } else { ")," },
            no_break: requires_comma_to_be_match_arm(body).unwrap().then_some(','),
            ..BreakToken::default()
        });
        printer.end();
    }

    printer.end();
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
        _ => Err(syn::Error::new(expr.span(), "unhandled expression")),
    }
}

#[cfg(test)]
mod test {
    use crate::macro_syntax::MacroFactory;
    use crate::macros::select::SelectFactory;
    use crate::{assert_eq_text, format_file};

    use syn::Macro;

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
        let _select = SelectFactory::parse(&mac).unwrap();
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
        let _select = SelectFactory::parse(&mac).unwrap();
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
        let _select = SelectFactory::parse(&mac).unwrap();
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
        let _select = SelectFactory::parse(&mac).unwrap();
    }

    #[test]
    fn test_replace_select_braced_expr() {
        let code = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => {
                stmt();
                a.expr()
            }
            val2 = future2.expr() => {
                stmt();
                b.expr()
            }
        }
    }
}
        "#;

        let formatted = format_file(code);

        assert_eq_text!(formatted, code);
    }

    #[test]
    fn test_replace_select_unbraced_expr() {
        let code = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => a.expr(),
            val2 = future2.expr() => b.expr(),
        }
    }
}
        "#;

        let formatted = format_file(code);

        assert_eq_text!(formatted, code);
    }

    #[test]
    fn test_replace_select_unbrace() {
        let code = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => {
                a.expr()
            }
            val2 = future2.expr() => {
                b.expr()
            }
        }
    }
}
        "#;

        let formatted = format_file(code);

        let expected_formatted = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => a.expr(),
            val2 = future2.expr() => b.expr(),
        }
    }
}
        "#;

        assert_eq_text!(formatted, expected_formatted);
    }

    #[test]
    fn test_replace_select_brace() {
        let code = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => a.expr().expr().expr().expr().expr().expr().expr().expr().expr().expr(),
        }
    }
}
        "#;

        let formatted = format_file(code);

        let expected_formatted = r#"
fn func() {
    loop {
        select! {
            val1 = future1.expr() => {
                a.expr().expr().expr().expr().expr().expr().expr().expr().expr().expr()
            }
        }
    }
}
        "#;

        assert_eq_text!(formatted, expected_formatted);
    }
}
