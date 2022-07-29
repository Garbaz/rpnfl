use crate::ast::Expr;

#[derive(Clone)]
pub struct Config {
    pub trace: bool,
    pub stack: bool,
}

pub fn reduce(config: &Config, expr: Expr) -> Option<Expr> {
    reduce_(config, Vec::new(), expr)
}

#[derive(Debug, Clone)]
struct Trace {
    pub arg: Expr,
    pub rec: Expr,
}

fn reduce_(config: &Config, mut stack: Vec<Trace>, expr: Expr) -> Option<Expr> {
    if config.trace {
        if config.stack {
            println!("{:?}", stack);
        }
        println!("| {}", expr);
    }
    match expr {
        Expr::Argument(n) => {
            let Trace { arg, .. } = stack.get(n)?;
            Some(arg.clone())
        }
        Expr::Recursion(_) => Some(expr),
        Expr::Abstraction(_) => Some(expr),
        Expr::Application(a, f) => {
            let a = reduce_(config, stack.clone(), *a)?;
            stack.push(Trace {
                arg: a,
                rec: *f.clone(),
            });
            evaluate(config, stack.clone(), *f)
        }
    }
}

fn evaluate(config: &Config, mut stack: Vec<Trace>, expr: Expr) -> Option<Expr> {
    // if config.trace {
    //     if config.stack {
    //         println!("{:?}", stack);
    //     }
    //     println!(">| {}", expr);
    // }
    match expr {
        Expr::Abstraction(b) => evaluate(config, stack, *b),
        Expr::Recursion(m) => {
            let t = stack.pop()?;
            let mut stack = stack.get(stack.len() - m..)?.to_vec();
            let Trace { rec, .. } = stack.get(0)?.clone();
            stack.push(t);
            reduce_(config, stack, rec)
        }
        _ => reduce_(config, stack, expr),
    }
}