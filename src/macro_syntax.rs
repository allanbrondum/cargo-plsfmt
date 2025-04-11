pub trait MacroSyntax {
    fn delimiter_replacement(&self, base_indent: isize) -> String;
}

pub trait MacroFactory {
    /// Name the macro binds to
    fn name() -> &'static str;

    fn parse(mac: &syn::Macro) -> Option<Box<dyn MacroSyntax>>;
}
