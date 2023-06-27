use crate::syntax::SyntaxKindDescriptor;
use colored::*;

#[derive(Debug, Clone)]
pub struct Diagnostics {
    pub errors: Vec<ErrorKind>,
    pub input: String,
}

impl Diagnostics {
    pub fn new(input: String) -> Self {
        Self {
            errors: Vec::new(),
            input,
        }
    }

    pub fn add_error(&mut self, error: ErrorKind) {
        self.errors.push(error);
    }

    pub fn merge(&mut self, diagnostics: Diagnostics) {
        self.errors.extend(diagnostics.errors);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn print(&self) {
        for error in &self.errors {
            match error {
                ErrorKind::UnexpectedToken {
                    expected,
                    found: _,
                    position,
                } => {
                    let range_start = if *position as isize - 10 > 0 {
                        *position - 10
                    } else {
                        0
                    };
                    let range_end = if *position + 10 < self.input.len() {
                        *position + 10
                    } else {
                        self.input.len()
                    };

                    let start = &self.input[range_start..*position];
                    let character = &self.input[*position..*position];

                    let mut tokens = String::from(start);
                    tokens.push_str(&character.red().bold().to_string().underline());
                    tokens.push_str(&self.input[*position..range_end]);

                    println!(
                        "{}: Unexpected token {} found at position {}. Expected {}.",
                        "Error".red().bold(),
                        tokens.bold().red(),
                        position,
                        format!("{:?}", expected.syntax()).green().bold()
                    );
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    UnknownToken {
        token: SyntaxKindDescriptor,
        position: usize,
    },
    UnexpectedToken {
        expected: SyntaxKindDescriptor,
        found: SyntaxKindDescriptor,
        position: usize,
    },
    UnexpectedEndOfFile {
        expected: SyntaxKindDescriptor,
        position: usize,
    },
    ExpectedToken {
        expected: SyntaxKindDescriptor,
        found: SyntaxKindDescriptor,
        position: usize,
    },
    ExpectedExpression {
        found: SyntaxKindDescriptor,
        position: usize,
    },
    ExpectedIdentifier {
        found: SyntaxKindDescriptor,
        position: usize,
    },
    ExpectedEquals {
        found: SyntaxKindDescriptor,
        position: usize,
    },
    ExpectedSemicolon {
        found: SyntaxKindDescriptor,
        position: usize,
    },
    ParserError {
        token: SyntaxKindDescriptor,
        position: usize,
    },
    LexerError {
        token: SyntaxKindDescriptor,
        position: usize,
    },
    InvalidCharacterError {
        token: SyntaxKindDescriptor,
        position: usize,
    },
}
