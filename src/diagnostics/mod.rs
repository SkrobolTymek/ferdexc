use crate::ast::lexer::{TextSpan, TokenKind};

pub enum DiagnosticKind{
    Error,
    Warning,
}


pub struct Diagnostic {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(message: String, span: TextSpan, kind: DiagnosticKind) -> Self {
        Diagnostic { message, span, kind }
    }
}

pub struct DiagnosticBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag{
    pub fn new() -> Self {
        DiagnosticBag {
            diagnostics: vec![],
        }
    } 
    pub fn report_error(&mut self, message: String, span: TextSpan) {
        let error = Diagnostic::new(message, span, DiagnosticKind::Error);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        let warning = Diagnostic::new(message, span, DiagnosticKind::Warning);
        self.diagnostics.push(warning);
    }

    pub fn report_unexpected_token(&mut self, token: &Token, expected: &TokenKind, span: TextSpan) {
        self.report_error(format!("Expected <{}>, found <{}>", expected, token.kind), span);
    }

}
