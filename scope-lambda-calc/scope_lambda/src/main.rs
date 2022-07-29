use std::collections::VecDeque;

use dialoguer::{theme::ColorfulTheme, History, Input};
use scope_lambda::reduce;

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

fn main() {
    let mut prettyprint = false;
    let mut input_hist = InputHistory(VecDeque::new());
    loop {
        let line: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("λ")
            .history_with(&mut input_hist)
            .interact_text()
            .unwrap();

        if line == "@" {
            prettyprint = !prettyprint;
            if prettyprint {
                println!("Switched to pretty-printing AST");
            } else {
                println!("Switched to printing Lambda expression")
            }
        } else {
            let parse = parse::ExprParser::new().parse(&line);
            if prettyprint {
                println!("{:#?}", parse);
            } else {
                match &parse {
                    Ok(l) => println!("{}", l),
                    Err(e) => println!("{:?}", e),
                }
            }
            if let Ok(l) = parse {
                if let Some(l) = reduce::reduce(*l) {
                    println!("=>>\n{}", l);
                }
            }
        }
    }
}
