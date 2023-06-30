use crate::syntax::SyntaxKind;

use super::{EvaluationResult, Types};

pub fn evaluate(
    left: SyntaxKind,
    right: SyntaxKind,
    operator: SyntaxKind,
    variables: &mut Vec<(String, Types)>,
) -> EvaluationResult {
    let mut left = super::evaluate(left, variables);
    let mut right = super::evaluate(right, variables);
    let left_clone = left.clone();
    let right_clone = right.clone();

    loop {
        match left {
            EvaluationResult::Syntax(syntax) => {
                left = super::evaluate(syntax, variables);
            }
            _ => {}
        }
        match right {
            EvaluationResult::Syntax(syntax) => {
                right = super::evaluate(syntax, variables);
            }
            _ => {
                break;
            }
        }
    }

    match operator {
        SyntaxKind::Token(crate::syntax::Tokens::PlusToken) => match (left, right) {
            (EvaluationResult::String(left), EvaluationResult::String(right)) => {
                EvaluationResult::String(format!("{}{}", left, right))
            }
            (EvaluationResult::Int(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Int(left + right)
            }
            (EvaluationResult::Float(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left + right)
            }
            (EvaluationResult::Int(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left as f64 + right)
            }

            (EvaluationResult::Float(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Float(left + right as f64)
            }

            _ => {
                panic!("Cannot add {:?} and {:?}", left_clone, right_clone);
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::MinusToken) => match (left, right) {
            (EvaluationResult::Int(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Int(left - right)
            }
            (EvaluationResult::Float(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left - right)
            }
            (EvaluationResult::Int(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left as f64 - right)
            }

            (EvaluationResult::Float(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Float(left - right as f64)
            }
            _ => {
                panic!("Cannot subtract {:?} and {:?}", left_clone, right_clone);
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::StarToken) => match (left, right) {
            (EvaluationResult::Int(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Int(left * right)
            }
            (EvaluationResult::Float(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left * right)
            }
            (EvaluationResult::Int(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left as f64 * right)
            }

            (EvaluationResult::Float(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Float(left * right as f64)
            }
            _ => {
                panic!("Cannot multiply {:?} and {:?}", left_clone, right_clone);
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::SlashToken) => match (left, right) {
            (EvaluationResult::Float(a), EvaluationResult::Float(b)) => {
                if b == 0.0 {
                    panic!("Cannot divide by zero");
                }
                return EvaluationResult::Float(a / b);
            }
            (EvaluationResult::Int(a), EvaluationResult::Int(b)) => {
                if b == 0 {
                    panic!("Cannot divide by zero");
                }
                EvaluationResult::Float(a as f64 / b as f64)
            }
            (EvaluationResult::Int(a), EvaluationResult::Float(b)) => {
                if b == 0.0 {
                    panic!("Cannot divide by zero");
                }
                EvaluationResult::Float(a as f64 / b)
            }
            (EvaluationResult::Float(a), EvaluationResult::Int(b)) => {
                if b == 0 {
                    panic!("Cannot divide by zero");
                }
                EvaluationResult::Float(a / b as f64)
            }
            _ => {
                panic!("Cannot divide {:?} and {:?}", left_clone, right_clone);
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::PercentToken) => match (left, right) {
            (EvaluationResult::Int(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Int(left % right)
            }
            (EvaluationResult::Float(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left % right)
            }
            (EvaluationResult::Int(left), EvaluationResult::Float(right)) => {
                EvaluationResult::Float(left as f64 % right)
            }
            (EvaluationResult::Float(left), EvaluationResult::Int(right)) => {
                EvaluationResult::Float(left % right as f64)
            }
            _ => {
                panic!("Cannot modulo {:?} and {:?}", left_clone, right_clone);
            }
        },
        SyntaxKind::Token(crate::syntax::Tokens::BangEqualsToken) => match (left, right) {
            (EvaluationResult::Boolean(a), EvaluationResult::Boolean(b)) => {
                EvaluationResult::Boolean(a != b)
            },
            _ => {
                panic!("Cannot compare {:?} and {:?}", left_clone, right_clone);
            }
        }
        SyntaxKind::Token(crate::syntax::Tokens::AmpersandAmpersandToken) => match (left, right) {
            (EvaluationResult::Boolean(left), EvaluationResult::Boolean(right)) => {
                EvaluationResult::Boolean(left && right)
            }
            _ => {
                panic!(
                    "Cannot evaluate binary expression with operator: {:?} on types {:?} and {:?}",
                    operator, left_clone, right_clone
                );
            }
        }
        _ => {
            panic!(
                "Cannot evaluate binary expression with operator: {:?} on types {:?} and {:?}",
                operator, left_clone, right_clone
            );
        }
    }
}
