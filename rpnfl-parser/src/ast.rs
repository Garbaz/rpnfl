// pub enum ObjectKind {
//     Module,
//     Function,
//     Constructor,
// }

// pub struct Object<'a> {
//     pub kind : ObjectKind,
//     pub name : String,
//     pub parent : &'a Object<'a>,
// }
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Expr {
    pub location: (usize, usize),
    pub raw_expr: RawExpr,
}

impl Expr {
    pub fn new(l: usize, r: usize, raw_expr: RawExpr) -> Self {
        Self {
            location: (l, r),
            raw_expr,
        }
    }
}

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(fmt, "Expr {{\n{:?}\n}}", self.raw_expr)
//     }
// }

// impl Display for Expr {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(fmt, "{:#?}", self.raw_expr)
//     }
// }

#[derive(Debug, Clone)]
pub enum RawExpr {
    SubExpressions(Vec<Expr>),
    EscExpressions(Vec<Expr>),
    Reference(Path),
    Assign(String),
    Tag(String),
    DefFunction {
        name: String,
        froms: TypeList,
        to: Type,
        body: Box<Expr>,
    },
    DefConstructor {
        name: String,
        froms: TypeList,
    },
    DefModule {
        name: String,
        froms: TypeList,
        body: Box<Expr>,
    },
    Destructor {
        pattern: Pattern,
        body: Box<Expr>,
    },
    Conditional {
        if_: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Collection(ExprList),
    Numeral(Numeral),
    // Bespoke(Box<Bespoke>),
    Import(Path),
}

#[derive(Debug, Clone)]
pub struct Path(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct ExprList {
    pub ordered: bool,
    pub exprs: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct Type {
    pub location: (usize, usize),
    pub raw_type: RawType,
}

impl Type {
    pub fn new(l: usize, r: usize, raw_type: RawType) -> Self {
        Self {
            location: (l, r),
            raw_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RawType {
    Tagged { tag: String, type_: Box<Type> },
    Function { froms: TypeList, to: Box<Type> },
    Variable(String),
    DataType { path: Path, args: Vec<Type> },
}

#[derive(Debug, Clone)]
pub struct TypeList {
    pub ordered: bool,
    pub types: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Blank,
    Variable(String),
    Constructor { name: String, args: Vec<Pattern> },
    Tagged { tag: String, arg: Box<Pattern> },
}

#[derive(Debug, Clone)]
pub enum Numeral {
    Integer(i64),
    Float(f64),
}

// #[derive(Debug, Clone)]
// pub enum Bespoke {
//     Add(Expr, Expr),
//     Sub(Expr, Expr),
//     Mul(Expr, Expr),
//     Div(Expr, Expr),
//     Neg(Expr),
//     And(Expr, Expr),
//     Or(Expr, Expr),
//     Not(Expr, Expr),
// }
