extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use pest::Parser;
use pest::error::Error;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use std::fs;

#[derive(Parser)]
#[grammar = "pistolet.pest"]
pub struct PistoletParser;

#[derive(Debug)]
enum PistoletValue<'a> {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Var(&'a str)
}

#[derive(Debug)]
enum PistoletExpr<'a> {
    VAL(PistoletValue<'a>),
    Add(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Sub(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Mul(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Div(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    And(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Orb(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Xor(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>),
    Eq(Box<PistoletExpr<'a>>, Box<PistoletExpr<'a>>)
}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(sub, Left),
            Operator::new(mul, Left) | Operator::new(div, Left),
            Operator::new(and, Left) | Operator::new(or, Left) | Operator::new(xor, Left),
            Operator::new(eq, Left)
        ])
    };
}

#[derive(Debug)]
enum PistoletAST<'a> {
    Let(&'a str, &'a str, PistoletExpr<'a>),
    If(PistoletExpr<'a>, Box<PistoletAST<'a>>, Box<PistoletAST<'a>>),
    Seq(Vec<PistoletAST<'a>>),
    EOI
}

fn main() {
    let unparsed_file = fs::read_to_string("test/test_parser.pst")
                            .expect("cannot read file");
    // let pistolet_prog = PistoletParser::parse(Rule::program, &unparsed_file)
    //                         .expect("unsuccessful parse")
    //                         .next().unwrap();
    // println!("{:#?}", pistolet_prog);
    println!("{:#?}", parse_to_ast(&unparsed_file));
}

fn parse_to_ast(file: &str) -> Result<PistoletAST, Error<Rule>> {
    let pistolet_prog = PistoletParser::parse(Rule::program, &file)
                            .expect("unsuccessful parse")
                            .next().unwrap();
    use pest::iterators::Pair;
    use pest::iterators::Pairs;

    fn parse_value(pair: Pair<Rule>) -> PistoletValue {
        match pair.as_rule() {
            Rule::INTEGER => PistoletValue::Integer(pair.as_str().parse().unwrap()),
            Rule::FLOAT => PistoletValue::Float(pair.as_str().parse().unwrap()),
            Rule::BOOL => PistoletValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::VAR_NAME => PistoletValue::Var(pair.as_str()),

            _ => unreachable!()
        }
    }

    fn parse_expr(exp: Pairs<Rule>) -> PistoletExpr {
        PREC_CLIMBER.climb(
            exp,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::VALUE => PistoletExpr::VAL(parse_value(pair.into_inner().peek().unwrap())),
                Rule::EXPR => parse_expr(pair.into_inner()),
                Rule::BOOL_VALUE => PistoletExpr::VAL(parse_value(pair.into_inner().peek().unwrap())),
                Rule::BOOL_EXPR => parse_expr(pair.into_inner()),
                Rule::EQ_EXPR => parse_expr(pair.into_inner()),
                _ => {println!("{:#?}",pair); unreachable!()}
            },
            |lhs: PistoletExpr, op: Pair<Rule>, rhs: PistoletExpr| match op.as_rule() {
                Rule::and      => PistoletExpr::And(Box::new(lhs),Box::new(rhs)),
                Rule::or => PistoletExpr::Orb(Box::new(lhs),Box::new(rhs)),
                Rule::xor => PistoletExpr::Xor(Box::new(lhs),Box::new(rhs)),
                Rule::add      => PistoletExpr::Add(Box::new(lhs),Box::new(rhs)),
                Rule::sub      => PistoletExpr::Sub(Box::new(lhs),Box::new(rhs)),
                Rule::mul      => PistoletExpr::Mul(Box::new(lhs),Box::new(rhs)),
                Rule::div      => PistoletExpr::Div(Box::new(lhs),Box::new(rhs)),
                Rule::eq => PistoletExpr::Eq(Box::new(lhs),Box::new(rhs)),
                _ => unreachable!()
            }
        )
    }

    fn parse_prog(pair: Pair<Rule>) -> PistoletAST {
        match pair.as_rule() {
            Rule::program => PistoletAST::Seq(pair.into_inner().map(parse_prog).collect()),
            Rule::sentence => parse_prog(pair.into_inner().peek().unwrap()),
            Rule::LET => {
                let mut new_pair = pair.into_inner();
                PistoletAST::Let(
                    {let x = new_pair.next().unwrap();
                    match x.as_rule() {
                        Rule::VAR_NAME => x.as_str(),
                        Rule::TYPE_NAME => x.as_str(),
                        _ => unreachable!()
                    }},
                    {let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::VAR_NAME => x.as_str(),
                            Rule::TYPE_NAME => x.as_str(),
                            _ => unreachable!()
                    }},
                    match new_pair.peek().unwrap().as_rule() {
                        Rule::BOOL_EXPR => parse_expr(new_pair.peek().unwrap().into_inner()),
                        Rule::EXPR => parse_expr(new_pair.peek().unwrap().into_inner()),
                        _ => unreachable!()
                    }
                )
            }
            Rule::EOI => PistoletAST::EOI,
            _ => unimplemented!()
        }
    }

    Ok(parse_prog(pistolet_prog))
}