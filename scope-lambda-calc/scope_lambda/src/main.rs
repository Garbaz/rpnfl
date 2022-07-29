use std::collections::VecDeque;

use dialoguer::{theme::ColorfulTheme, History, Input};
use scope_lambda::reduce::{self, Config};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parse);

struct InputHistory(VecDeque<String>);

impl History<String> for InputHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.0.get(pos).cloned()
    }

    fn write(&mut self, val: &String) {
        self.0.push_front(val.clone())
    }
}

fn printhelp() {
    println!("  Commands:");
    println!("    ast - Enable/[Disable] prettyprinting AST.");
    println!("    trace - [Enable]/Disable printing the evaluation trace.");
    println!("    stack - Enable/[Disable] printing the stack in the evaluation trace.");
    println!();
    println!("  Otherwise, simply enter any valid expression to be evaluated.");
}

fn abled(b: bool) -> String {
    (if b { "enabled" } else { "disabled" }).to_string()
}

fn main() {
    let mut prettyprint = false;
    let mut input_hist = InputHistory(VecDeque::new());

    printhelp();

    let mut config = Config {
        trace: true,
        stack: false,
    };

    loop {
        let line: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("λ")
            .history_with(&mut input_hist)
            .interact_text()
            .unwrap();

        if line == "ast" {
            prettyprint = !prettyprint;
            println!("Pretty printing AST has been {}", abled(prettyprint));
        } else if line == "trace" {
            config.trace = !config.trace;
            println!(
                "Printing trace during evaluation has been {}",
                abled(config.trace)
            );
        } else if line == "stack" {
            config.stack = !config.stack;
            println!(
                "Printing stack when printing trace has been {}",
                abled(config.stack)
            );
        } else {
            let parse = parse::ExprParser::new().parse(&line);
            if prettyprint {
                println!("{:#?}", parse);
                println!();
            } else {
                match &parse {
                    Ok(l) => println!("{}", l),
                    Err(e) => println!("{:?}", e),
                }
            }
            if let Ok(l) = parse {
                if let Some(l) = reduce::reduce(&config, *l) {
                    println!("\n==>\n {}", l);
                }
            }
        }
    }
}
