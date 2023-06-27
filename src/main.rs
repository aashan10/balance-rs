
#![allow(dead_code)]

use evaluator::Types;

mod diagnostics;
mod evaluator;
mod lexer;
mod parser;
mod syntax;
fn main() {

    let mut show_tree = false;
    let mut show_stack = false;
    let mut variables: Vec<(String, Types)> = Vec::new();
    clearscreen::clear().expect("Could not clear screen");
    loop {

        println!("> ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "#clear" {
            clearscreen::clear().expect("Could not clear screen");
            variables.clear();
            continue;
        }

        if input == "#exit" {
            break;
        }
        if input == "#show_tree" {
            show_tree = !show_tree;
            continue;
        }

        if input == "#show_stack" {
            show_stack = !show_stack;
            continue;
        }

        let mut lexer = lexer::Lexer::new(input.to_string());
        let tokens = lexer.lex();

        if show_tree {
            print!("Lexer tokens: \n");
            for token in &tokens {
                token.print();
                print!("\n");
            }
        }

        let mut parser = parser::Parser::new(tokens, input.to_string());
        let tree = parser.parse();

        if show_tree {
            print!("\n\nSyntax tree: \n");
            tree.print();
        }


        let mut diagnostics = lexer.diagnostics();
        diagnostics.merge(parser.diagnostics());

        if diagnostics.has_errors() {
            diagnostics.print();
            continue;
        }

        if show_tree {
            print!("\n\nSyntax tree: \n");
            tree.print();
        }



        
        let evaluator = evaluator::evaluate(tree.syntax(), &mut variables);

        print!("\n\nResult: \n{:?}", evaluator);

        if show_stack {
            print!("\n\nStack: \n");
            for (name, value) in &variables {
                print!("{}: {:?}\n", name, value);
            }
        }
    }
}
