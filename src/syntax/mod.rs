#![allow(dead_code)]


#[derive(Clone, Debug, PartialEq)]
pub enum LiteralToken {
    Int { value: i64 },
    Float { value: f64 },
    String { value: String },
    Char { value: char },
    Bool { value: bool },
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    BadToken { value: String },
    EndOfFileToken,
    WhiteSpaceToken,
    LiteralToken { value: LiteralToken },
    NumberToken { value: f64 },
    IdentifierToken { value: String },
    CommentToken { value: String },
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    PercentToken,
    EqualsToken,
    EqualsEqualsToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    SemiColonToken,
    ColonToken,
    CommaToken,
    DotToken,
    BangToken,
    AmpersandToken,
    AmpersandAmpersandToken,
    PipeToken,
    PipePipeToken,
    CaretToken,
    TildeToken,
    QuestionMarkToken,
    LessThanToken,
    GreaterThanToken,
    HashToken,
    AtToken,
    DollarToken,
    BackSlashToken,
    SingleQuoteToken,
    DoubleQuoteToken,
    BackTickToken,
    NewLineToken,
    TabToken,
    OpenBracketToken,
    CloseBracketToken,
    OpenBraceToken,
    CloseBraceToken,


    // These are only here to be represented in diagnostics
    BinaryOperatorToken,
    UnaryOperatorToken,
    UnknownToken {value: String},
    
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keywords {
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    LoopKeyword,
    BreakKeyword,
    ContinueKeyword,
    MatchKeyword,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionSyntax {
    BinaryExpressionSyntax {
        left: Box<SyntaxKind>,
        operator: Box<SyntaxKind>,
        right: Box<SyntaxKind>,
    },
    ParenthesizedExpressionSyntax {
        open_parenthesis_token: Box<SyntaxKind>,
        expression: Box<SyntaxKind>,
        close_parenthesis_token: Box<SyntaxKind>,
    },
    UnaryExpressionSyntax {
        operator: Box<SyntaxKind>,
        operand: Box<SyntaxKind>,
    },
    LiteralExpressionSyntax {
        expression: Box<SyntaxKind>,
    },
    IdentifierExpressionSyntax { identifier: Box<SyntaxKind> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementSyntax {
    BlockStatementSyntax {
        open_brace: Box<SyntaxKind>,
        statements: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
    ExpressionStatementSyntax {
        expression: Box<SyntaxKind>,
        semicolon: Box<SyntaxKind>,
    },
    VariableAssignmentStatementSyntax {
        identifier: Box<SyntaxKind>,
        equals_token: Box<SyntaxKind>,
        expression: Box<SyntaxKind>,
    },
    VariableDeclarationStatementSyntax {
        keyword: Box<SyntaxKind>,
        identifier: Box<SyntaxKind>,
        equals_token: Box<SyntaxKind>,
        expression: Box<SyntaxKind>,
        semicolon: Box<SyntaxKind>,
    },
    IfStatementSyntax {
        keyword: Box<SyntaxKind>,
        open_parenthesis: Box<SyntaxKind>,
        condition: Box<SyntaxKind>,
        close_parenthesis: Box<SyntaxKind>,
        open_brace: Box<SyntaxKind>,
        body: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
    WhileStatementSyntax {
        keyword: Box<SyntaxKind>,
        open_parenthesis: Box<SyntaxKind>,
        condition: Box<SyntaxKind>,
        close_parenthesis: Box<SyntaxKind>,
        open_brace: Box<SyntaxKind>,
        body: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
    ForStatementSyntax {
        keyword: Box<SyntaxKind>,
        open_parenthesis: Box<SyntaxKind>,
        initializer: Box<SyntaxKind>,
        first_semicolon: Box<SyntaxKind>,
        condition: Box<SyntaxKind>,
        second_semicolon: Box<SyntaxKind>,
        incrementor: Box<SyntaxKind>,
        close_parenthesis: Box<SyntaxKind>,
        open_brace: Box<SyntaxKind>,
        body: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
    BreakStatementSyntax {
        keyword: Box<SyntaxKind>,
        label: Option<Box<SyntaxKind>>,
        semicolon: Box<SyntaxKind>,
    },
    ContinueStatementSyntax {
        keyword: Box<SyntaxKind>,
        label: Option<Box<SyntaxKind>>,
        semicolon: Box<SyntaxKind>,
    },
    ReturnStatementSyntax {
        keyword: Box<SyntaxKind>,
        expression: Option<Box<SyntaxKind>>,
        semicolon: Box<SyntaxKind>,
    },
    MatchStatementSyntax {
        keyword: Box<SyntaxKind>,
        open_parenthesis: Box<SyntaxKind>,
        expression: Box<SyntaxKind>,
        close_parenthesis: Box<SyntaxKind>,
        open_brace: Box<SyntaxKind>,
        arms: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxKind {
    Token(Tokens),
    Expression(ExpressionSyntax),
    Statement(StatementSyntax),
    Keyword(Keywords),
}

impl SyntaxKind {
    pub fn matches(&self, other: &SyntaxKind) -> bool {
        return self.eq(other);
    }

    pub fn from(kind: Box<SyntaxKind>) -> SyntaxKind {
        *kind.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxKindDescriptor {
    position: usize,
    syntax: SyntaxKind,
}

impl SyntaxKindDescriptor {
    pub fn new(position: usize, syntax: SyntaxKind) -> Self {
        Self { position, syntax }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn syntax(&self) -> SyntaxKind {
        self.syntax.clone()
    }

    pub fn print(&self) {
        let syntax = self.syntax();
        self.print_syntax_kind(syntax, None);
    }

    fn print_syntax_kind(&self, kind: SyntaxKind, prefix: Option<String>) {
        let prefix = match prefix {
            Some(prefix) => prefix,
            None => String::from(""),
        };
        match kind {
            SyntaxKind::Expression(ExpressionSyntax::BinaryExpressionSyntax { left, operator, right }) => {
                println!("{}BinaryExpressionSyntax", prefix);
                self.print_syntax_kind(*left, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*operator, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*right, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Expression(ExpressionSyntax::ParenthesizedExpressionSyntax { open_parenthesis_token, expression, close_parenthesis_token }) => {
                println!("{}ParenthesizedExpressionSyntax", prefix);
                self.print_syntax_kind(*open_parenthesis_token, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*expression, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_parenthesis_token, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Expression(ExpressionSyntax::UnaryExpressionSyntax { operator, operand }) => {
                println!("{}UnaryExpressionSyntax", prefix);
                self.print_syntax_kind(*operator, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*operand, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Expression(ExpressionSyntax::LiteralExpressionSyntax { expression }) => {
                println!("{}LiteralExpressionSyntax", prefix);
                self.print_syntax_kind(*expression, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Statement(StatementSyntax::BlockStatementSyntax { open_brace, statements, close_brace }) => {
                println!("{}BlockStatementSyntax", prefix);
                self.print_syntax_kind(*open_brace, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*statements, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_brace, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Statement(StatementSyntax::ExpressionStatementSyntax { expression, semicolon }) => {
                println!("{}ExpressionStatementSyntax", prefix);
                self.print_syntax_kind(*expression, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*semicolon, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Statement(StatementSyntax::VariableAssignmentStatementSyntax { identifier, equals_token, expression }) => {
                println!("{}VariableAssignmentStatementSyntax", prefix);
                self.print_syntax_kind(*identifier, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*equals_token, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*expression, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Statement(StatementSyntax::IfStatementSyntax { keyword, open_parenthesis, condition, close_parenthesis, open_brace, body, close_brace }) => {
                println!("{}IfStatementSyntax", prefix);
                self.print_syntax_kind(*keyword, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*open_parenthesis, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*condition, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_parenthesis, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*open_brace, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*body, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_brace, Some(format!("{}\t", prefix)));
            }
            SyntaxKind::Statement(StatementSyntax::ForStatementSyntax { keyword, open_parenthesis, initializer, first_semicolon, condition, second_semicolon, incrementor, close_parenthesis, open_brace, body, close_brace }) => {
                println!("{}ForStatementSyntax", prefix);
                self.print_syntax_kind(*keyword, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*open_parenthesis, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*initializer, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*first_semicolon, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*condition, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*second_semicolon, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*incrementor, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_parenthesis, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*open_brace, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*body, Some(format!("{}\t", prefix)));
                self.print_syntax_kind(*close_brace, Some(format!("{}\t", prefix)));
            }

            _ => {
                println!("{}{:?}", prefix, kind);
            }
        }
    }
}
