use std::{collections::VecDeque, fs};

use dialoguer::{theme::ColorfulTheme, History, Input};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parse);

// static mut PRINT_AST_WITHOUT_LOCATIONS: bool = false;

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
    println!("    :r FILE  -- Read and parse the file FILE.");
    println!();
    println!("  Otherwise, simply enter any valid expression to be evaluated.");
}

// fn abled(b: bool) -> String {
//     (if b { "enabled" } else { "disabled" }).to_string()
// }

fn parse_and_print(src: &str) {
    let parse = parse::ExprParser::new().parse(src);
    match &parse {
        Ok(p) => println!("{:#?}", p),
        Err(e) => println!("{}", e),
    }
    // if prettyprint {
    //     println!("{:#?}", parse);
    //     println!();
    // } else {
    //     match &parse {
    //         Ok(l) => println!("{:#?}", l),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }
    // if let Ok(l) = parse {}
}

fn interactive() -> ! {
    // let mut prettyprint = true;
    let mut input_hist = InputHistory(VecDeque::new());

    printhelp();

    loop {
        let line: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("ρ")
            .history_with(&mut input_hist)
            .interact_text()
            .unwrap();

        // println!("{}", line);

        if let ":" = &line[..1] {
            let mut cmd = (&line[1..]).split_whitespace();
            match cmd.next() {
                Some("r") => {
                    if let Some(f) = cmd.next() {
                        if let Ok(src) = fs::read_to_string(f) {
                            parse_and_print(&src);
                        } else {
                            println!(
                                "REPL Err: I could not read the file \"{}\"",
                                f
                            );
                        }
                    } else {
                        println!("REPL Err: I need a filename to open.");
                    }
                }
                // Some("l") => unsafe {
                //     PRINT_AST_WITHOUT_LOCATIONS = !PRINT_AST_WITHOUT_LOCATIONS;
                //     println!(
                //         "REPL Info: Printing AST without locations is now {}.",
                //         abled(PRINT_AST_WITHOUT_LOCATIONS)
                //     )
                // },
                _ => println!("REPL Err: I don't know this command."),
            }
        } else {
            parse_and_print(&line);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        interactive();
    } else if let Ok(src) = fs::read_to_string(&args[1]) {
        parse_and_print(&src);
    }
}
