use crate::model::ParsedMacro;

use syn::spanned::Spanned;

pub fn replace(content: &str, mut parsed_macros: Vec<ParsedMacro>) -> String {
    parsed_macros.sort_by_key(|parsed_macro| parsed_macro.syn_macro.span().start());

    let mut out = String::new();
    let mut cursor = 0;
    for parsed_macro in parsed_macros {
        out.push_str(
            &content[cursor
                ..parsed_macro
                    .syn_macro
                    .delimiter
                    .span()
                    .span()
                    .byte_range()
                    .start],
        );
        out.push_str(
            &parsed_macro
                .macro_syntax
                .delimiter_replacement(parsed_macro.syn_macro.path.span().start().column as isize),
        );
        cursor = parsed_macro
            .syn_macro
            .delimiter
            .span()
            .span()
            .byte_range()
            .end;
    }
    out.push_str(std::str::from_utf8(&content.as_bytes()[cursor..]).unwrap());

    out
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// struct OrderedPosition {
//     line: usize,
//     column: usize,
// }
//
// impl From<Position> for OrderedPosition {
//     fn from(value: Position) -> Self {
//         Self {
//             line: value.line,
//             column: value.column,
//         }
//     }
// }
//
// impl From<LineColumn> for OrderedPosition {
//     fn from(value: LineColumn) -> Self {
//         Self {
//             line: value.line,
//             column: value.column,
//         }
//     }
// }
//
// impl Ord for OrderedPosition {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.line
//             .cmp(&other.line)
//             .then(self.column.cmp(&other.column))
//     }
// }
//
// impl PartialOrd for OrderedPosition {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
