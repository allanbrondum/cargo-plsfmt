mod replacement;

use crate::replacement::Replacement;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Macro, visit};

#[derive(Default)]
struct MacroVisitor<'ast> {
    select_macros: Vec<&'ast Macro>,
}

impl<'ast> visit::Visit<'ast> for MacroVisitor<'ast> {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if mac
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident.eq("select"))
        {
            self.select_macros.push(mac);
        }
    }
}

pub fn format_file(content: &str) -> String {
    let file = syn::parse_file(content).unwrap();

    let mut visitor = MacroVisitor::default();
    visitor.visit_file(&file);

    let mut replacements = Vec::new();
    for mac in &visitor.select_macros {
        replacements.push(Replacement::new(mac.tokens.span(), String::new()));
    }

    replacement::replace(content, replacements)
}
