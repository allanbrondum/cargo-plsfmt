mod comments;
mod macro_factory_binding;
mod macro_syntax;
mod macros;
mod model;
mod replace;

use crate::macro_factory_binding::parse_macro_syntax;
use crate::model::ParsedMacro;
use syn::visit::Visit;
use syn::{Macro, visit};

#[derive(Default)]
struct MacroVisitor<'ast> {
    parsed_macros: Vec<ParsedMacro<'ast>>,
}

impl<'ast> visit::Visit<'ast> for MacroVisitor<'ast> {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if let Some(macro_syntax) = parse_macro_syntax(mac) {
            self.parsed_macros.push(ParsedMacro {
                macro_syntax,
                syn_macro: mac,
            });
        }
    }
}

pub fn format_file(content: &str) -> String {
    let file = syn::parse_file(content).unwrap();

    let mut visitor = MacroVisitor::default();
    visitor.visit_file(&file);

    // let comments = comments::scan_comments(content);

    replace::replace(content, visitor.parsed_macros)
}
