use proc_macro2::{LineColumn, Span};
use std::collections::HashMap;

pub struct Replacement {
    span: Span,
    replacement: String,
}

impl Replacement {
    pub fn new(span: Span, replacement: String) -> Self {
        Self { span, replacement }
    }
}

#[derive(Default)]
struct LineMap {
    map: HashMap<usize, usize>,
}

impl LineMap {
    fn idx(&self, line_col: LineColumn) -> usize {
        self.map.get(&line_col.line).expect("line") + line_col.column
    }
}

fn line_map(content: &str) -> LineMap {
    let mut ret = LineMap::default();

    let mut idx = 0;
    for (line_number, line) in content.lines().enumerate() {
        let line_number = line_number + 1;
        ret.map.insert(line_number, idx);
        idx += line.len() + 1;
    }
    ret
}

pub fn replace(content: &str, mut replacements: Vec<Replacement>) -> String {
    replacements.sort_by_key(|repl| repl.span.start());
    let line_map = line_map(content);

    let mut out = String::new();
    let mut cursor = 0;
    for repl in replacements {
        let start_idx = line_map.idx(repl.span.start());
        out.push_str(&content[cursor..start_idx]);
        out.push_str(&repl.replacement);
        cursor = line_map.idx(repl.span.end());
    }
    out.push_str(&content[cursor..]);

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::__private::ext::RepToTokensExt;
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
