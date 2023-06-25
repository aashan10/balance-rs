use crate::parser::ParserError;
use colored::*;

pub fn handle_parser_errors(errors: Vec<ParserError>, text: String) -> bool {
    if errors.len() > 0 {
        for error in errors {
            let message = error.message;
            let descriptor = error.descriptor;

            let position = descriptor.position();

            let range_start: usize = if position as isize - 10 > 0 {
                position - 10
            } else {
                0
            };
            let range_end: usize = if position + 10 < text.len() {
                position + 10
            } else {
                text.len()
            };

            let range = &text[range_start..range_end];

            let message = format!(
                "{} {}. ...{}... at position {}",
                "Error: ".red(),
                message,
                range.bold().red(),
                position.to_string().bold()
            );
            println!("{}\n", message);
        }
        return false
    }
    true
}
