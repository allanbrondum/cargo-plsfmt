mod macros;
mod replacement;

use crate::macros::select;
use syn::visit::Visit;
use syn::{Macro, visit};

#[derive(Default)]
struct MacroVisitor<'ast> {
    select_macros: Vec<&'ast Macro>,
}

impl<'ast> visit::Visit<'ast> for MacroVisitor<'ast> {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if let Some(ident) = mac.path.segments.last().map(|seg| &seg.ident) {
            if ident == "select" {
                self.select_macros.push(mac);
            }
        }
    }
}

pub fn format_file(content: &str) -> String {
    let file = syn::parse_file(content).unwrap();

    let mut visitor = MacroVisitor::default();
    visitor.visit_file(&file);

    let mut replacements = Vec::new();
    for mac in &visitor.select_macros {
        replacements.extend(select::select_replace(mac));
    }

    replacement::replace(content, replacements)
}
