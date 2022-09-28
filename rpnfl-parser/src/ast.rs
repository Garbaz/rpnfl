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

#[derive(Debug, Clone)]
pub struct Path(pub Vec<String>);

#[derive(Debug, Clone)]
pub enum Expr {
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
    Destructor(Pattern, Box<Expr>),
    Conditional {
        if_: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Collection(ExprList),
    Numeral(Numeral),
    Bespoke(Bespoke),
    Import(Path),
}

#[derive(Debug, Clone)]
pub struct ExprList {
    pub ordered: bool,
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Tagged { tag: String, type_: Box<Type> },
    Function { froms: TypeList, to: Box<Type> },
    Variable(String),
    DataType { path : Path, args: Vec<Type>} ,
}

#[derive(Debug, Clone)]
pub struct TypeList {
    pub ordered: bool,
    pub types: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Numeral {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum Bespoke {}

#[derive(Debug, Clone)]
pub enum Pattern {}
