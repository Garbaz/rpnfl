use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Expr {
    Argument(usize),
    Recursion(usize),
    Abstraction(Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Argument(n) => write!(fmt, "${}", n),
            Expr::Recursion(l) => write!(fmt, "%{}", l),
            Expr::Abstraction(b) => write!(fmt, "{{{}}}", b),
            Expr::Application(a, f) => {
                if let Expr::Application(_, _) = a.as_ref() {
                    write!(fmt, "({}) ", a)?
                } else {
                    write!(fmt, "{} ", a)?
                }
                if let Expr::Application(_, _) = f.as_ref() {
                    write!(fmt, "({})", f)
                } else {
                    write!(fmt, "{}", f)
                }
            }
        }
    }
}
