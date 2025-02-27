use proc_macro2::Span;

pub struct Replacement {
    pub span: Span,
    pub replacement: String,
}

impl Replacement {
    pub fn new(span: Span, replacement: String) -> Self {
        Self { span, replacement }
    }
}

pub fn replace(content: &str, mut replacements: Vec<Replacement>) -> String {
    replacements.sort_by_key(|repl| repl.span.start());

    let mut out = String::new();
    let mut cursor = 0;
    for repl in replacements {
        out.push_str(&content[cursor..repl.span.byte_range().start]);
        out.push_str(&repl.replacement);
        cursor = repl.span.byte_range().end;
    }
    out.push_str(std::str::from_utf8(&content.as_bytes()[cursor..]).unwrap());

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    #[test]
    fn replace_none() {
        let content = r#"
aaa aa aaaa
bbb bb
        "#;

        let res = replace(content, vec![]);
        assert_eq!(res, content);
    }

    #[test]
    fn replace_single_line() {
        let content = r#"
aaa aa aaaa
bbb bb
        "#;
        let tks: Vec<_> = TokenStream::from_str(content)
            .unwrap()
            .into_iter()
            .collect();

        let res = replace(
            content,
            vec![Replacement::new(
                tks[1].span(),
                "ccccc".to_ascii_lowercase(),
            )],
        );
        let expected = r#"
aaa ccccc aaaa
bbb bb
        "#;
        assert_eq!(res, expected);
    }

    #[test]
    fn replace_multi_line() {
        let content = r#"
aaa aa aaaa
bbb bb
        "#;
        let tks: Vec<_> = TokenStream::from_str(content)
            .unwrap()
            .into_iter()
            .collect();

        let res = replace(
            content,
            vec![Replacement::new(
                tks[1].span().join(tks[3].span()).unwrap(),
                "ccccc".to_ascii_lowercase(),
            )],
        );
        let expected = r#"
aaa ccccc bb
        "#;
        assert_eq!(res, expected);
    }
}
