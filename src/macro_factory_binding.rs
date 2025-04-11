use crate::macro_syntax::{MacroFactory, MacroSyntax};
use crate::macros::select::SelectFactory;

macro_rules! return_macro_syntax {
    ($ident:ident, $syn_macro:ident, $supplier_factory:ty) => {
        if $ident == <$supplier_factory>::name() {
            if let Some(macro_syntax) = <$supplier_factory>::parse($syn_macro) {
                return Some(macro_syntax);
            }
        }
    };
}

pub fn parse_macro_syntax(syn_macro: &syn::Macro) -> Option<Box<dyn MacroSyntax>> {
    let ident = syn_macro.path.segments.last().map(|seg| &seg.ident)?;

    return_macro_syntax!(ident, syn_macro, SelectFactory);

    None
}
