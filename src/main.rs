use crate::error_handler::handle_parser_errors;

mod lexer;
mod parser;
mod syntax;
mod error_handler;
fn main() {
    let input = String::from("let x = 5 + 5;");
    let mut lexer = lexer::Lexer::new(input.clone());
    let tokens = lexer.lex();

    let mut parser = parser::Parser::new(tokens);
    let _tree = parser.parse();

    let errors = parser.errors();
    let errors_handled = handle_parser_errors(errors, input);
    
    if !errors_handled {
        return;
    }
    
}
