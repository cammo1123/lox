use std::{env, process::exit};
use crafting_interpreters::ast_printer::AstPrinter;
use crafting_interpreters::r#gen::expr::Expr;
use crafting_interpreters::run_file;
use crafting_interpreters::run_prompt;
use crafting_interpreters::token::Token;
use crafting_interpreters::token::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let expression = Expr::Binary { 
        left: Box::new(Expr::Unary { 
            operator: Token::new(TokenType::Minus, "-", 1),
            right: Box::new(Expr::Literal { value: "123".to_owned() })
        }),
        operator: Token::new(TokenType::Star, "*", 1),
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal { value: "45.67".to_owned() }) })
    };

    let mut ast_printer = AstPrinter::new();
    println!("{}", ast_printer.print(&expression));

    match args.len() {
        1 => {
            if let Err(e) = run_prompt() {
                eprintln!("REPL Error: {}", e);
                exit(70);
            }
        }

        2 => {
            if let Err(e) = run_file(&args[1]) {
                eprintln!("Error running file: {}", e);
                exit(65);
            }
        }

        _ => {
            println!("Usage: jlox [script]");
            exit(64);
        }
    }
}
