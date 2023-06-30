use crate::{
    syntax::{
        SyntaxKind::{self, Token},
        Tokens
    },
};

pub fn binary_operator_precedence(kind: SyntaxKind) -> usize {
    match kind {
        Token(Tokens::PlusToken) => 1,
        Token(Tokens::MinusToken) => 1,

        Token(Tokens::AmpersandAmpersandToken) => 2,
        Token(Tokens::PipePipeToken) => 2,

        Token(Tokens::StarToken) => 3,
        Token(Tokens::SlashToken) => 3,

        Token(Tokens::PercentToken) => 4,
        Token(Tokens::BangEqualsToken) => 4,
        _ => 0,
    }
}

pub fn unary_operator_precedence(kind: SyntaxKind) -> usize {
    match kind {
        Token(Tokens::BangToken) => 6,
        Token(Tokens::PlusToken) => 5,
        Token(Tokens::MinusToken) => 5,
        _ => 0,
    }
}
