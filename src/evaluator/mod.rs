use crate::syntax::{self, SyntaxKind};
mod binary_expression_evaluator;
mod unary_expression_evaluator;

#[derive(Debug)]
pub enum EvaluationResult {
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Syntax(SyntaxKind),
    Null,
}

impl Clone for EvaluationResult {
    fn clone(&self) -> Self {
        match self {
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Char(arg0) => Self::Char(arg0.clone()),
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Float(arg0) => Self::Float(arg0.clone()),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Syntax(arg0) => Self::Syntax(arg0.clone()),
            Self::Null => Self::Null,
        }
    }
}


#[derive(Debug)]
pub enum Types {
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Clone for Types {
    fn clone(&self) -> Self {
        match self {
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Char(arg0) => Self::Char(arg0.clone()),
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Float(arg0) => Self::Float(arg0.clone()),
            Self::Boolean(arg0) => Self::Boolean(arg0.clone()),
            Self::Null => Self::Null,
        }
    }
}


pub fn evaluate(syntax: SyntaxKind, variables: &mut Vec<(String, Types)>) -> EvaluationResult {
    match syntax {
        SyntaxKind::Expression(crate::syntax::ExpressionSyntax::BinaryExpressionSyntax {
            left,
            operator,
            right,
        }) => {
            binary_expression_evaluator::evaluate(
                *left,
                *right,
                *operator,
                variables,
            )
        }

        SyntaxKind::Statement(
            crate::syntax::StatementSyntax::VariableAssignmentStatementSyntax {
                identifier,
                equals_token: _,
                expression,
            },
        ) => {
            let mut expression = evaluate(*expression, variables);

            loop {
                match expression {
                    EvaluationResult::Syntax(syntax) => {
                        expression = evaluate(syntax, variables);
                    }
                    _ => {
                        break;
                    }
                }
            }

            match *identifier {
                SyntaxKind::Token(crate::syntax::Tokens::IdentifierToken { value }) => {
                    let mut filtered_variables: Vec<(String, Types)> = variables
                        .iter()
                        .filter(|(name, _)| *name != value)// Filter out elements with variable name "variable2"
                        .cloned()
                        .collect();

                    match expression {
                        EvaluationResult::String(s) => {
                            filtered_variables.push((value, Types::String(s)));
                        }
                        EvaluationResult::Int(s) => {
                            filtered_variables.push((value, Types::Int(s)));
                        }
                        EvaluationResult::Boolean(s) => {
                            filtered_variables.push((value, Types::Boolean(s)));
                        }
                        EvaluationResult::Null => {
                            filtered_variables.push((value, Types::Null));
                        }
                        _ => {
                            panic!("Cannot declare variable with expression: {:?}", expression);
                        }
                    };

                    *variables = filtered_variables;
                }
                _ => {
                    panic!("Cannot declare variable with identifier: {:?}", identifier);
                }
            }

            EvaluationResult::Null
        }

        SyntaxKind::Statement(
            crate::syntax::StatementSyntax::VariableDeclarationStatementSyntax {
                keyword: _,
                identifier,
                equals_token: _,
                expression,
                semicolon: _,
            },
        ) => {
            let mut expression = evaluate(*expression, variables);

            loop {
                match expression {
                    EvaluationResult::Syntax(syntax) => {
                        expression = evaluate(syntax, variables);
                    }
                    _ => {
                        break;
                    }
                }
            }

            match *identifier {
                SyntaxKind::Token(crate::syntax::Tokens::IdentifierToken { value }) => {
                    match expression {
                        EvaluationResult::String(s) => {
                            variables.push((value, Types::String(s)));
                        }
                        EvaluationResult::Int(s) => {
                            variables.push((value, Types::Int(s)));
                        }
                        EvaluationResult::Float(s) => {
                            variables.push((value, Types::Float(s)));
                        }
                        EvaluationResult::Char(s) => {
                            variables.push((value, Types::Char(s)));
                        }

                        EvaluationResult::Boolean(s) => {
                            variables.push((value, Types::Boolean(s)));
                        }
                        EvaluationResult::Null => {
                            variables.push((value, Types::Null));
                        }
                        _ => {
                            panic!("Cannot declare variable with expression: {:?}", expression);
                        }
                    }
                }
                _ => {
                    panic!("Cannot declare variable with identifier: {:?}", identifier);
                }
            }
            EvaluationResult::Null
        }

        SyntaxKind::Expression(crate::syntax::ExpressionSyntax::LiteralExpressionSyntax {
            expression,
        }) => match *expression {
            SyntaxKind::Token(crate::syntax::Tokens::LiteralToken { value })
            => {
                match value {
                    syntax::LiteralToken::Int { value } => EvaluationResult::Int(value),
                    syntax::LiteralToken::Float { value } => EvaluationResult::Float(value),
                    syntax::LiteralToken::String { value } => EvaluationResult::String(value),
                    syntax::LiteralToken::Char { value } => EvaluationResult::Char(value),
                    syntax::LiteralToken::Bool { value } => EvaluationResult::Boolean(value),
                    syntax::LiteralToken::Null => EvaluationResult::Null,
                }
            }
            SyntaxKind::Token(crate::syntax::Tokens::IdentifierToken { value }) => {
                let (_, var_type) = variables
                    .iter()
                    .find(|(name, _)| *name == value)
                    .unwrap();

                match var_type {
                    Types::String(s) => EvaluationResult::String(s.clone()),
                    Types::Int(s) => EvaluationResult::Int(*s),
                    Types::Float(s) => EvaluationResult::Float(*s),
                    Types::Char(s) => EvaluationResult::Char(*s),
                    Types::Boolean(s) => EvaluationResult::Boolean(*s),
                    Types::Null => EvaluationResult::Null,
                }
            }
            SyntaxKind::Keyword(crate::syntax::Keywords::TrueKeyword) => {
                EvaluationResult::Boolean(true)
            }
            SyntaxKind::Keyword(crate::syntax::Keywords::FalseKeyword) => {
                EvaluationResult::Boolean(false)
            }
            SyntaxKind::Keyword(crate::syntax::Keywords::NullKeyword) => EvaluationResult::Null,
            _ => {
                panic!("Cannot evaluate literal expression: {:?}", expression);
            }
        },

        SyntaxKind::Expression(
            crate::syntax::ExpressionSyntax::ParenthesizedExpressionSyntax {
                open_parenthesis_token: _,
                close_parenthesis_token: _,
                expression,
            },
        ) => evaluate(*expression, variables),

        SyntaxKind::Expression(syntax::ExpressionSyntax::UnaryExpressionSyntax {
            operator,
            operand,
        }) => {
            unary_expression_evaluator::evaluate(*operator, *operand, variables)
        }

        _ => {
            panic!("Cannot evaluate syntax kind: {:?}", syntax);
        }
    }
}
