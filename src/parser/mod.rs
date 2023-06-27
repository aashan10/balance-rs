mod precedence;

use core::panic;

use crate::{
    diagnostics::Diagnostics,
    syntax::{
        ExpressionSyntax::{BinaryExpressionSyntax, UnaryExpressionSyntax},
        Keywords, LiteralToken,
        SyntaxKind::{self, Keyword, Token},
        SyntaxKindDescriptor, Tokens,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParserError {
    pub descriptor: SyntaxKindDescriptor,
    pub message: String,
}
pub struct Parser {
    tokens: Vec<SyntaxKindDescriptor>,
    position: usize,
    diagnostics: Diagnostics,
}

impl Parser {
    pub fn new(tokens: Vec<SyntaxKindDescriptor>, input: String) -> Parser {
        Parser {
            tokens,
            position: 0,
            diagnostics: Diagnostics::new(input),
        }
    }

    fn peek(&self, offset: usize) -> SyntaxKindDescriptor {
        let position = self.position + offset;
        if position >= self.tokens.len() {
            return SyntaxKindDescriptor::new(self.position, Token(Tokens::EndOfFileToken));
        }
        self.tokens[position].clone()
    }

    fn current(&self) -> SyntaxKindDescriptor {
        self.peek(0)
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> SyntaxKindDescriptor {
        let current = self.current().clone();
        self.next();
        current
    }

    fn match_token(&mut self, descriptor: SyntaxKindDescriptor) -> SyntaxKindDescriptor {
        let kind = descriptor.syntax();
        if self.current().syntax().matches(&kind) {
            return self.next_token();
        }

        self.diagnostics
            .add_error(crate::diagnostics::ErrorKind::UnexpectedToken {
                expected: descriptor,
                found: self.current(),
                position: self.current().position(),
            });

        SyntaxKindDescriptor::new(self.position, kind)
    }

    fn parse_expression(&mut self, parent_precedence: Option<usize>) -> SyntaxKindDescriptor {
        let parent_precedence = parent_precedence.unwrap_or(0);
        let current = self.current().clone();
        let unary_precedence = precedence::unary_operator_precedence(current.syntax());
        let mut left = if unary_precedence != 0 && unary_precedence >= parent_precedence {
            let operator = self.next_token().syntax();
            let operand = self.parse_expression(None);
            SyntaxKindDescriptor::new(
                current.position(),
                SyntaxKind::Expression(UnaryExpressionSyntax {
                    operand: Box::new(operand.syntax()),
                    operator: Box::new(operator),
                }),
            )
        } else {
            self.parse_primary_expression()
        };

        loop {
            let precedence = precedence::binary_operator_precedence(self.current().syntax());
            if precedence == 0 || precedence <= parent_precedence {
                break;
            }

            let operator = self.next_token().syntax();
            let right = self.parse_expression(Some(precedence));
            left = SyntaxKindDescriptor::new(
                current.position(),
                SyntaxKind::Expression(BinaryExpressionSyntax {
                    left: Box::new(left.syntax()),
                    operator: Box::new(operator),
                    right: Box::new(right.syntax()),
                }),
            );
        }

        left
    }

    fn parse_primary_expression(&mut self) -> SyntaxKindDescriptor {
        let current = self.current().clone();
        let result = match current.syntax() {
            Token(Tokens::OpenParenthesisToken) => {
                let left = self.next_token();
                let expression = self.parse_expression(None);
                let right = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::CloseParenthesisToken),
                ));
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::ParenthesizedExpressionSyntax {
                            open_parenthesis_token: Box::new(left.syntax()),
                            expression: Box::new(expression.syntax()),
                            close_parenthesis_token: Box::new(right.syntax()),
                        },
                    ),
                )
            }
            Token(Tokens::LiteralToken {
                value: LiteralToken::Int { value: _ },
            }) => {
                let number = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                            expression: Box::new(number.syntax()),
                        },
                    ),
                )
            }
            Token(Tokens::LiteralToken {
                value: LiteralToken::Float { value: _ },
            }) => {
                let number = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                            expression: Box::new(number.syntax()),
                        },
                    ),
                )
            }

            Token(Tokens::LiteralToken {
                value: LiteralToken::String { value: _ },
            }) => {
                let string = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                            expression: Box::new(string.syntax()),
                        },
                    ),
                )
            }

            Token(Tokens::LiteralToken {
                value: LiteralToken::Bool { value: _ },
            }) => {
                let boolean = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                            expression: Box::new(boolean.syntax()),
                        },
                    ),
                )
            }

            Token(Tokens::LiteralToken { value: _ }) => {
                let left = self.next_token();
                let operator = self.next_token();
                let right = self.parse_expression(None);

                match operator.syntax() {
                    Token(Tokens::PlusToken)
                    | Token(Tokens::MinusToken)
                    | Token(Tokens::StarToken)
                    | Token(Tokens::SlashToken)
                    | Token(Tokens::PercentToken) => SyntaxKindDescriptor::new(
                        current.position(),
                        SyntaxKind::Expression(BinaryExpressionSyntax {
                            left: Box::new(left.syntax()),
                            operator: Box::new(operator.syntax()),
                            right: Box::new(right.syntax()),
                        }),
                    ),
                    Token(Tokens::EqualsToken) => SyntaxKindDescriptor::new(
                        current.position(),
                        SyntaxKind::Statement(
                            crate::syntax::StatementSyntax::VariableAssignmentStatementSyntax {
                                identifier: Box::new(left.syntax()),
                                equals_token: Box::new(operator.syntax()),
                                expression: Box::new(right.syntax()),
                            },
                        ),
                    ),
                    Token(Tokens::AmpersandAmpersandToken) => {
                        let right = self.parse_expression(None);
                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Expression(BinaryExpressionSyntax {
                                left: Box::new(left.syntax()),
                                operator: Box::new(operator.syntax()),
                                right: Box::new(right.syntax()),
                            }),
                        )
                    }

                    _ => {
                        self.diagnostics.add_error(
                            crate::diagnostics::ErrorKind::UnexpectedToken {
                                expected: SyntaxKindDescriptor::new(
                                    self.position,
                                    Token(Tokens::BinaryOperatorToken),
                                ),
                                position: operator.position(),
                                found: operator,
                            },
                        );
                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Token(Tokens::BadToken {
                                value: "Bad Token".to_string(),
                            }),
                        )
                    }
                }
            }

            Token(Tokens::IdentifierToken { value }) => {
                let identifier = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(
                        crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                            expression: Box::new(identifier.syntax()),
                        },
                    ),
                )
            }

            Token(Tokens::OpenBraceToken) => {
                let statements = self.parse_primary_expression();
                let close_bracket = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::CloseBraceToken),
                ));

                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Statement(crate::syntax::StatementSyntax::BlockStatementSyntax {
                        open_brace: Box::new(current.syntax()),
                        statements: Box::new(statements.syntax()),
                        close_brace: Box::new(close_bracket.syntax()),
                    }),
                )
            }

            Keyword(Keywords::LetKeyword) => {
                let let_keyword = self.next_token();
                let identifier = self.next_token();
                let identifier_syntax = identifier.syntax();

                match identifier_syntax {
                    SyntaxKind::Token(Tokens::IdentifierToken { value: _ }) => {
                        let equals_token = self.match_token(SyntaxKindDescriptor::new(
                            self.position,
                            Token(Tokens::EqualsToken),
                        ));
                        let expression = self.parse_expression(None);
                        let semicolon_token = self.match_token(SyntaxKindDescriptor::new(
                            self.position,
                            Token(Tokens::SemiColonToken),
                        ));
                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Statement(crate::syntax::StatementSyntax::VariableDeclarationStatementSyntax {
                                keyword: Box::new(let_keyword.syntax()),
                                identifier: Box::new(identifier.syntax()),
                                equals_token: Box::new(equals_token.syntax()),
                                expression: Box::new(expression.syntax()),
                                semicolon: Box::new(semicolon_token.syntax()),
                            }),
                        )
                    }
                    _ => {
                        self.diagnostics.add_error(
                            crate::diagnostics::ErrorKind::UnexpectedToken {
                                expected: SyntaxKindDescriptor::new(
                                    identifier.position(),
                                    Token(Tokens::LiteralToken {
                                        value: crate::syntax::LiteralToken::String {
                                            value: "id".to_string(),
                                        },
                                    }),
                                ),
                                position: identifier.position(),
                                found: identifier,
                            },
                        );

                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Token(Tokens::BadToken {
                                value: "Bad Token".to_string(),
                            }),
                        )
                    }
                }
            }
            Keyword(Keywords::IfKeyword) => {
                let if_keyword = self.next_token();
                let open_parenthesis = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::OpenParenthesisToken),
                ));
                let condition = self.parse_expression(None);
                let close_parenthesis = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::CloseParenthesisToken),
                ));
                let open_braces = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::OpenBraceToken),
                ));
                let body = self.parse_expression(None);
                let close_braces = self.match_token(SyntaxKindDescriptor::new(
                    self.position,
                    Token(Tokens::CloseBraceToken),
                ));
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Statement(crate::syntax::StatementSyntax::IfStatementSyntax {
                        keyword: Box::new(if_keyword.syntax()),
                        open_brace: Box::new(open_braces.syntax()),
                        close_brace: Box::new(close_braces.syntax()),
                        open_parenthesis: Box::new(open_parenthesis.syntax()),
                        close_parenthesis: Box::new(close_parenthesis.syntax()),
                        condition: Box::new(condition.syntax()),
                        body: Box::new(body.syntax()),
                    }),
                )
            }

            _ => {
                let descriptor = self.current();
                let position = descriptor.position();
                self.diagnostics
                    .add_error(crate::diagnostics::ErrorKind::UnexpectedToken {
                        expected: SyntaxKindDescriptor::new(
                            self.position,
                            Token(Tokens::LiteralToken {
                                value: crate::syntax::LiteralToken::String {
                                    value: "identifier".to_string(),
                                },
                            }),
                        ),
                        position,
                        found: descriptor,
                    });

                SyntaxKindDescriptor::new(
                    position,
                    SyntaxKind::Token(Tokens::BadToken {
                        value: "Bad Token".to_string(),
                    }),
                )
            }
        };
        result
    }

    pub fn parse(&mut self) -> SyntaxKindDescriptor {
        let expression = self.parse_expression(None);
        self.match_token(SyntaxKindDescriptor::new(
            self.position,
            Token(Tokens::EndOfFileToken),
        ));
        expression
    }

    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics.clone()
    }
}
