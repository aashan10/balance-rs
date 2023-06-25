#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    BadToken {value: String},
    EndOfFileToken,
    WhiteSpaceToken,
    NumberToken {value: f64},
    AlphaNumericToken {value: String},
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    PercentToken,
    EqualsToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    SemiColonToken,
    ColonToken,
    CommaToken,
    DotToken,
    BangToken,
    AmpersandToken,
    PipeToken,
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
    LiteralExpressionSyntax { expression: Box<SyntaxKind> },
    VariableAssignmentSyntax,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementSyntax {
    BlockStatementSyntax {
        open_brace: Box<SyntaxKind>,
        statements: Box<SyntaxKind>,
        close_brace: Box<SyntaxKind>,
    },
    ExpressionStatementSyntax,
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
    WhileStatementSyntax,
    ForStatementSyntax,
    BreakStatementSyntax,
    ContinueStatementSyntax,
    ReturnStatementSyntax,
    MatchStatementSyntax,
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
}