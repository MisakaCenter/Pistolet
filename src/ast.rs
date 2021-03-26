
#[derive(Debug)]
enum PistoletValue<'a> {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Var(&'a str),
    Funcall(&'a str, Vec<PistoletExpr<'a>>)
}

#[derive(Debug)]
enum PistoletExpr<'a> {
    Val(PistoletValue<'a>),
    Add(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Sub(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Mul(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Div(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    And(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Orb(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Xor(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Eq(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
}

#[derive(Debug)]
enum PistoletAST<'a> {
    Seq(Vec<PistoletAST<'a>>),
    Let(&'a str, &'a str, PistoletExpr<'a>),
    If(PistoletExpr<'a>, Box<PistoletAST<'a>>, Box<PistoletAST<'a>>),
    While(PistoletExpr<'a>, Box<PistoletAST<'a>>),
    Return(PistoletExpr<'a>),
    Varbind(&'a str, &'a str),
    Paralist(Vec<PistoletAST<'a>>),
    Fun(&'a str, Box<PistoletAST<'a>>, &'a str, Box<PistoletAST<'a>>),
    EOI
}