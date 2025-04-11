use crate::ast::lexer::TextSpan;

pub enum DiagnosticKind{
    Error,
    Warning,
}


pub struct Diagnostics {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostics {
    pub fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
        Self { message, span, kind }
    }
}