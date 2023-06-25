use crate::syntax::{
    ExpressionSyntax::{BinaryExpressionSyntax, UnaryExpressionSyntax},
    Keywords,
    SyntaxKind::{self, Keyword, Token},
    SyntaxKindDescriptor, Tokens,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParserError {
    pub descriptor: SyntaxKindDescriptor,
    pub message: String,
}
pub struct Parser {
    tokens: Vec<SyntaxKindDescriptor>,
    position: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<SyntaxKindDescriptor>) -> Parser {
        Parser {
            tokens,
            position: 0,
            errors: Vec::new(),
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

    fn match_token(&mut self, kind: SyntaxKind) -> SyntaxKindDescriptor {
        if self.current().syntax().matches(&kind) {
            return self.next_token();
        }
        self.errors.push(ParserError {
            descriptor: self.current(),
            message: format!("Expected {:?}, got {:?}", kind, self.current().syntax()),
        });
        SyntaxKindDescriptor::new(self.position, kind)
    }

    fn parse_expression(&mut self, parent_precedence: Option<usize>) -> SyntaxKindDescriptor {
        let parent_precedence = parent_precedence.unwrap_or(0);
        let current = self.current().clone();
        let unary_precedence = unary_operator_precedence(current.syntax());
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
            let precedence = binary_operator_precedence(self.current().syntax());
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
                let right = self.match_token(Token(Tokens::CloseParenthesisToken));
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(crate::syntax::ExpressionSyntax::ParenthesizedExpressionSyntax {
                        open_parenthesis_token: Box::new(left.syntax()),
                        expression: Box::new(expression.syntax()),
                        close_parenthesis_token: Box::new(right.syntax()),
                    }),
                )
            },
            Token(Tokens::AlphaNumericToken { value: _ }) => {
                let left = self.next_token();
                let operator = self.next_token();
                let right = self.parse_expression(None);
                
                match operator.syntax() {
                    Token(Tokens::PlusToken) | 
                    Token(Tokens::MinusToken) |
                    Token(Tokens::StarToken) |
                    Token(Tokens::SlashToken) |
                    Token(Tokens::PercentToken) => {
                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Expression(BinaryExpressionSyntax {
                                left: Box::new(left.syntax()),
                                operator: Box::new(operator.syntax()),
                                right: Box::new(right.syntax()),
                            }),
                        )
                    },
                    _ => {
                        self.errors.push(ParserError {
                            message: String::from(format!("Expected a binary operator, got {:?}", operator.syntax())),
                            descriptor: operator,
                        });
                        SyntaxKindDescriptor::new(
                            current.position(),
                            
                            SyntaxKind::Token(Tokens::BadToken { value: String::from("Bad Token") })
                        )
                    }
                }
            },

            Token(Tokens::NumberToken { value: _ }) => {
                let number = self.next_token();
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Expression(crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
                        expression: Box::new(number.syntax()),
                    }),
                )
            },
            Token(Tokens::OpenBraceToken) => {
                let statements = self.parse_primary_expression();
                let close_bracket = self.match_token(Token(Tokens::CloseBraceToken));

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
                    SyntaxKind::Token(Tokens::AlphaNumericToken { value: _ }) => {
                        let equals_token = self.match_token(Token(Tokens::EqualsToken));
                        let expression = self.parse_expression(None);
                        let semicolon_token = self.match_token(Token(Tokens::SemiColonToken));
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
                    },
                     _ => {
                        self.errors.push(ParserError {
                            message: String::from(format!("Expected an identifier, got {:?}", identifier.syntax())),
                            descriptor: identifier,
                        });
                        SyntaxKindDescriptor::new(
                            current.position(),
                            SyntaxKind::Token(Tokens::BadToken { value: String::from("Bad Token") })
                        )
                     }
                    
                }
            },
            Keyword(Keywords::IfKeyword) => {
                let if_keyword = self.next_token();
                let open_parenthesis = self.match_token(Token(Tokens::OpenParenthesisToken));
                let condition = self.parse_expression(None);
                let close_parenthesis = self.match_token(Token(Tokens::CloseParenthesisToken));
                let open_braces = self.match_token(Token(Tokens::OpenBraceToken));
                let body = self.parse_expression(None);
                let close_braces = self.match_token(Token(Tokens::CloseBraceToken));
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
                self.errors.push(ParserError { 
                    descriptor: self.current(), 
                    message: format!("Unexpected token {:?}", self.current().syntax()) 
                });
                SyntaxKindDescriptor::new(
                    current.position(),
                    SyntaxKind::Token(Tokens::BadToken { value: String::from("Bad Token") })
                )
            }
        };
        result
    } 

    pub fn parse(&mut self) -> SyntaxKindDescriptor {
        
        let expression = self.parse_expression(None);
        self.match_token(Token(Tokens::EndOfFileToken));
        expression
    }

    pub fn errors(&self) -> Vec<ParserError> {
        self.errors.clone()
    }
}

fn binary_operator_precedence(kind: SyntaxKind) -> usize {
    match kind {
        Token(Tokens::PlusToken) => 1,
        Token(Tokens::MinusToken) => 1,
        Token(Tokens::StarToken) => 2,
        Token(Tokens::SlashToken) => 2,
        Token(Tokens::PercentToken) => 3,
        _ => 0,
    }
}

fn unary_operator_precedence(kind: SyntaxKind) -> usize {
    match kind {
        Token(Tokens::PlusToken) => 3,
        Token(Tokens::MinusToken) => 3,
        _ => 0,
    }
}
