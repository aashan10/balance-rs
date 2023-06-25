#![allow(dead_code)]
use crate::syntax::{Tokens, Keywords, SyntaxKind::{Token, Keyword}, SyntaxKindDescriptor};


#[derive(Debug, Clone)]
pub struct LexerError {
    pub descriptor: SyntaxKindDescriptor,
    pub message: String,
}
pub struct Lexer {
    input: String,
    position: usize,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            errors: Vec::new(),
        }
    }

    fn peek(&self, offset: usize) -> char {
        let position = self.position + offset;
        if position >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(position).unwrap()
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> SyntaxKindDescriptor {

        if self.current().is_numeric() {
            let mut value = String::new();
            while self.current().is_numeric() {
                value.push(self.current());
                self.next();
            }
            return SyntaxKindDescriptor::new(self.position, Token(Tokens::NumberToken { value: value.parse::<f64>().unwrap() }));
        }

        match self.current() {
            '\0' => SyntaxKindDescriptor::new(self.position, Token(Tokens::EndOfFileToken)),
            ' ' | '\n' | '\t' => {
                while self.current() == ' ' || self.current() == '\n' || self.current() == '\t' {
                    self.next();
                }
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::WhiteSpaceToken));
            }
            '+' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::PlusToken));
            }
            '-' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::MinusToken));
            }
            '*' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::StarToken));
            }
            '/' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::SlashToken));
            }
            '%' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::PercentToken));
            }
            '=' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::EqualsToken));
            }
            '(' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::OpenParenthesisToken));
            }
            ')' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::CloseParenthesisToken));
            }
            ';' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::SemiColonToken));
            }
            ':' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::ColonToken));
            }
            '{' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::OpenBraceToken));
            }
            '}' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::CloseBraceToken));
            }
            '[' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::OpenBracketToken));
            }
            ']' => {
                self.next();
                return SyntaxKindDescriptor::new(self.position, Token(Tokens::CloseBracketToken));
            }
            _ => {
                let mut value = String::new();
                let position = self.position;
                while self.current() != '\0' && self.current() != ' ' && self.current() != '\t' {
                    value.push(self.current());
                    self.next();
                }

                for character in value.chars() {
                    if !character.is_alphanumeric() && character != '_' {
                        let token_descriptor = SyntaxKindDescriptor::new(position, Token(Tokens::BadToken { value }));
                        let error = LexerError {
                            descriptor: token_descriptor.clone(),
                            message: format!("Invalid character: {}", character),
                        };
                        self.errors.push(error);
                        return token_descriptor;
                    }
                }

                match value.as_str() {
                    "let" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::LetKeyword)),
                    "if" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::IfKeyword)),
                    "else" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::ElseKeyword)),
                    "for" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::ForKeyword)),
                    "loop" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::LoopKeyword)),
                    "break" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::BreakKeyword)),
                    "continue" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::ContinueKeyword)),
                    "match" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::MatchKeyword)),
                    _ => {
                        return SyntaxKindDescriptor::new(position, Token(Tokens::AlphaNumericToken { value }));
                    }
                }
            }
        }
    }

    pub fn errors(&self) -> Vec<LexerError> {
        self.errors.clone()
    }
    
    pub fn lex(&mut self) -> Vec<SyntaxKindDescriptor> {
        let mut tokens: Vec<SyntaxKindDescriptor> = vec![];
        loop {
            let token = self.next_token();
            let syntax = token.syntax();

            match syntax {
                Token(Tokens::EndOfFileToken) => {
                    tokens.push(token);
                    break;
                }
                Token(Tokens::WhiteSpaceToken) => {
                    continue;
                }
                _ => {
                    tokens.push(token);
                }
            };
        }  
        tokens
    }


    
}