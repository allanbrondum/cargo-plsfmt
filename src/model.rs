use crate::macro_syntax::MacroSyntax;

pub struct ParsedMacro<'ast> {
    pub macro_syntax: Box<dyn MacroSyntax>,
    pub syn_macro: &'ast syn::Macro,
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Single line comment
pub struct Comment<'s> {
    pub position: Position,
    pub comment: &'s str,
}

impl<'s> Comment<'s> {
    pub fn new(position: Position, comment: &'s str) -> Self {
        Self { position, comment }
    }
}
