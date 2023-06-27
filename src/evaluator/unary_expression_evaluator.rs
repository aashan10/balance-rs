use crate::syntax::SyntaxKind;

use super::{EvaluationResult, Types};

pub fn evaluate(
    operator: SyntaxKind,
    operand: SyntaxKind,
    variables: &mut Vec<(String, Types)>,
) -> EvaluationResult {
    let mut operand = super::evaluate(operand, variables);

    loop {
        match operand {
            EvaluationResult::Syntax(syntax) => {
                operand = super::evaluate(syntax, variables);
            }
            _ => {
                break;
            }
        }
    }

    match operator {
        SyntaxKind::Token(crate::syntax::Tokens::PlusToken) => match operand {
            EvaluationResult::Int(operand) => EvaluationResult::Int(operand),
            EvaluationResult::Float(operand) => EvaluationResult::Float(operand),
            _ => {
                panic!(
                    "Cannot evaluate unary expression with operator: {:?}",
                    operator
                );
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::MinusToken) => match operand {
            EvaluationResult::Int(operand) => EvaluationResult::Int(-operand),
            EvaluationResult::Float(operand) => EvaluationResult::Float(-operand),
            _ => {
                panic!(
                    "Cannot evaluate unary expression with operator: {:?}",
                    operator
                );
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::BangToken) => match operand {
            EvaluationResult::Boolean(operand) => EvaluationResult::Boolean(!operand),
            _ => {
                panic!(
                    "Cannot evaluate unary expression with operator: {:?}",
                    operator
                );
            }
        },
        _ => {
            panic!(
                "Cannot evaluate unary expression with operator: {:?}",
                operator
            );
        }
    }
}
