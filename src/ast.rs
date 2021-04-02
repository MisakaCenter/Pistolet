#[derive(Debug, Clone)]
enum PistoletValue {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Var(String),
    Funcall(String, Vec<PistoletExpr>),
}

#[derive(Debug, Clone)]
enum PistoletExpr {
    Val(PistoletValue),
    Add(Box<PistoletExpr>, Box<PistoletExpr>),
    Sub(Box<PistoletExpr>, Box<PistoletExpr>),
    Mul(Box<PistoletExpr>, Box<PistoletExpr>),
    Div(Box<PistoletExpr>, Box<PistoletExpr>),
    And(Box<PistoletExpr>, Box<PistoletExpr>),
    Orb(Box<PistoletExpr>, Box<PistoletExpr>),
    Nand(Box<PistoletExpr>, Box<PistoletExpr>),
    Eq(Box<PistoletExpr>, Box<PistoletExpr>),
    Leq(Box<PistoletExpr>, Box<PistoletExpr>),
    Req(Box<PistoletExpr>, Box<PistoletExpr>),
    Left(Box<PistoletExpr>, Box<PistoletExpr>),
    Right(Box<PistoletExpr>, Box<PistoletExpr>)
}

#[derive(Debug, Clone)]
enum PistoletAST {
    Seq(Vec<PistoletAST>),
    Let(String, String, PistoletExpr),
    If(PistoletExpr, Box<PistoletAST>, Box<PistoletAST>),
    While(Box<PistoletAST>, PistoletExpr),
    Return(PistoletExpr),
    Varbind(String, String),
    Paralist(Vec<PistoletAST>),
    Fun(String, Box<PistoletAST>, String, Box<PistoletAST>),
    PrintLine(PistoletExpr),
    EOI,
}
