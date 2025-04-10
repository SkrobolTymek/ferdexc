pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

pub enum ASTStatementKind{
    expression(ASTExpression),
}



pub struct ASTStatement{
kind: ASTStatementKind
}

pub enum ASTExpressionKind{
    Number(i64),
}

pub struct ASTExpression{
    kind : ASTExpressionKind,
}