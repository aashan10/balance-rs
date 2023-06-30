#![allow(dead_code)]

pub mod source_text;

use crate::{
    diagnostics::{self, Diagnostics},
    syntax::{
        Keywords, LiteralToken,
        SyntaxKind::{Keyword, Token},
        SyntaxKindDescriptor, Tokens,
    },
};

#[derive(Debug, Clone)]
pub struct LexerError {
    pub descriptor: SyntaxKindDescriptor,
    pub message: String,
}
pub struct Lexer {
    input: source_text::SourceText,
    position: usize,
    diagnostics: Diagnostics,
}

impl Lexer {
    pub fn new(input: source_text::SourceText) -> Self {
        let diagnostics = diagnostics::Diagnostics::new(input.to_string());
        Self {
            input,
            position: 0,
            diagnostics,
        }
    }

    fn peek(&self, offset: usize) -> char {
        let position = self.position + offset;
        if position >= self.input.text.len() {
            return '\0';
        }
        self.input.text.chars().nth(position).unwrap()
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn lookahead(&self) -> char {
        self.peek(1)
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> SyntaxKindDescriptor {
        let start = self.position;

        match self.current() {
            '\0' => SyntaxKindDescriptor::new(start, Token(Tokens::EndOfFileToken)),
            ' ' | '\n' | '\t' => {
                while self.current() == ' ' || self.current() == '\n' || self.current() == '\t' {
                    self.next();
                }
                return SyntaxKindDescriptor::new(start, Token(Tokens::WhiteSpaceToken));
            }
            '#' => {
                let mut comment = String::from("");
                while self.current() != '\n' && self.current() != '\0' {
                    comment.push(self.current());
                    self.next();
                }
                return SyntaxKindDescriptor::new(
                    start,
                    Token(Tokens::CommentToken { value: comment }),
                );
            }
            '+' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::PlusToken));
            }
            '-' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::MinusToken));
            }
            '*' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::StarToken));
            }
            '/' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::SlashToken));
            }
            '%' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::PercentToken));
            }
            '!' => {
                self.next();
                if self.lookahead() == '=' {
                    self.next();
                    return SyntaxKindDescriptor::new(start, Token(Tokens::BangEqualsToken));
                }
                return SyntaxKindDescriptor::new(start, Token(Tokens::BangToken));
            }
            '=' => {
                self.next();
                if self.lookahead() == '=' {
                    self.next();
                    return SyntaxKindDescriptor::new(start, Token(Tokens::EqualsEqualsToken));
                }
                return SyntaxKindDescriptor::new(start, Token(Tokens::EqualsToken));
            }
            '(' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::OpenParenthesisToken));
            }
            ')' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::CloseParenthesisToken));
            }
            ';' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::SemiColonToken));
            }
            ':' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::ColonToken));
            }
            '{' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::OpenBraceToken));
            }
            '}' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::CloseBraceToken));
            }
            '[' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::OpenBracketToken));
            }
            ']' => {
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::CloseBracketToken));
            }
            '&' => {
                
                if self.lookahead() == '&' {
                    self.position += 2;
                    return SyntaxKindDescriptor::new(
                        start,
                        Token(Tokens::AmpersandAmpersandToken),
                    );
                }
                self.next();
                return SyntaxKindDescriptor::new(start, Token(Tokens::AmpersandToken));
            }
            '|' => {
                self.next();
                if self.lookahead() == '|' {
                    self.next();
                    return SyntaxKindDescriptor::new(start, Token(Tokens::PipePipeToken));
                }
                return SyntaxKindDescriptor::new(start, Token(Tokens::PipeToken));
            }

            '\'' => {
                self.next();
                let character = self.current();
                self.next();
                let closing = self.current();

                if closing != '\'' {
                    self.diagnostics
                        .add_error(diagnostics::ErrorKind::ExpectedToken {
                            expected: SyntaxKindDescriptor::new(
                                start,
                                Token(Tokens::SingleQuoteToken),
                            ),

                            found: SyntaxKindDescriptor::new(
                                self.position,
                                Token(Tokens::LiteralToken {
                                    value: LiteralToken::Char { value: character },
                                }),
                            ),
                            position: self.position,
                        });
                }

                return SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::LiteralToken {
                        value: LiteralToken::Char { value: character },
                    }),
                );
            }
            '"' => {
                self.next();
                let position = start;
                let mut value = String::new();
                while self.current() != '"' {
                    value.push(self.current());
                    self.next();
                }
                self.next();
                return SyntaxKindDescriptor::new(
                    position,
                    Token(Tokens::LiteralToken {
                        value: LiteralToken::String { value },
                    }),
                );
            }
            '0'..='9' => {
                let mut value = String::new();
                let mut is_float = false;
                while self.current().is_numeric()
                    || self.current() == '.'
                    || self.current() == 'e'
                    || self.current() == '-'
                    || self.current() == '+'
                {
                    if self.current() == '.' {
                        is_float = true;
                    }
                    value.push(self.current());
                    self.next();
                }

                if is_float {
                    return SyntaxKindDescriptor::new(
                        start,
                        Token(Tokens::LiteralToken {
                            value: LiteralToken::Float {
                                value: value.parse::<f64>().unwrap(),
                            },
                        }),
                    );
                } else {
                    return SyntaxKindDescriptor::new(
                        start,
                        Token(Tokens::LiteralToken {
                            value: LiteralToken::Int {
                                value: value.parse::<i64>().unwrap(),
                            },
                        }),
                    );
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut value = String::new();
                let position = start;
                while self.current().is_alphanumeric() || self.current() == '_' {
                    value.push(self.current());
                    self.next();
                }

                self.match_keywords_and_string_literals(value.as_str(), position)
            }
            _ => {
                let token = Tokens::UnknownToken {
                    value: self.current().to_string(),
                };
                self.diagnostics
                    .add_error(diagnostics::ErrorKind::UnknownToken {
                        token: SyntaxKindDescriptor::new(self.position, Token(token.clone())),
                        position: self.position,
                    });
                return SyntaxKindDescriptor::new(self.position, Token(token));
            }
        }
    }

    fn match_keywords_and_string_literals(
        &self,
        token: &str,
        position: usize,
    ) -> SyntaxKindDescriptor {
        match token {
            "let" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::LetKeyword)),
            "if" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::IfKeyword)),
            "else" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::ElseKeyword)),
            "for" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::ForKeyword)),
            "loop" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::LoopKeyword)),
            "break" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::BreakKeyword)),
            "continue" => {
                return SyntaxKindDescriptor::new(position, Keyword(Keywords::ContinueKeyword))
            }
            "match" => return SyntaxKindDescriptor::new(position, Keyword(Keywords::MatchKeyword)),
            "true" =>  SyntaxKindDescriptor::new(
                self.position,
                Token(Tokens::LiteralToken {
                    value: LiteralToken::Bool { value: true},
                })
            ),
            "false" =>  SyntaxKindDescriptor::new(
                self.position,
                Token(Tokens::LiteralToken {
                    value: LiteralToken::Bool { value: false},
                })
            ),
            _ => {
                return SyntaxKindDescriptor::new(
                    position,
                    Token(Tokens::IdentifierToken {
                        value: token.to_string(),
                    }),
                );
            }
        }
    }

    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics.clone()
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
