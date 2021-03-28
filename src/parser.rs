include!("ast.rs");

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use pest::error::Error;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "pistolet.pest"]
pub struct PistoletParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;
        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(sub, Left),
            Operator::new(mul, Left) | Operator::new(div, Left),
            Operator::new(and, Left) | Operator::new(or, Left) | Operator::new(nand, Left),
            Operator::new(eq, Left),
        ])
    };
}

fn parse_to_ast(file: &str) -> Result<PistoletAST, Error<Rule>> {
    let pistolet_prog = PistoletParser::parse(Rule::program, &file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    use pest::iterators::Pair;
    use pest::iterators::Pairs;

    fn parse_value(pair: Pair<Rule>) -> PistoletValue {
        match pair.as_rule() {
            Rule::INTEGER => PistoletValue::Integer(pair.as_str().parse().unwrap()),
            Rule::FLOAT => PistoletValue::Float(pair.as_str().parse().unwrap()),
            Rule::BOOL => PistoletValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::VAR_NAME => PistoletValue::Var(pair.as_str().to_string()),
            Rule::FUN_CALL => {
                let mut new_pair = pair.into_inner();
                PistoletValue::Funcall(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::FUN_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    new_pair.map(unwarp_expr).map(parse_expr).collect(),
                )
            }
            _ => unreachable!(),
        }
    }

    fn unwarp_expr(exp: Pair<Rule>) -> Pairs<Rule> {
        exp.into_inner()
    }

    fn parse_expr(exp: Pairs<Rule>) -> PistoletExpr {
        PREC_CLIMBER.climb(
            exp,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::VALUE => PistoletExpr::Val(parse_value(pair.into_inner().peek().unwrap())),
                Rule::EXPR => parse_expr(pair.into_inner()),
                Rule::BOOL_VALUE => {
                    PistoletExpr::Val(parse_value(pair.into_inner().peek().unwrap()))
                }
                Rule::BOOL_EXPR => parse_expr(pair.into_inner()),
                Rule::EXPR_NoTy => parse_expr(pair.into_inner()),
                Rule::EQ_EXPR => parse_expr(pair.into_inner()),
                _ => {
                    println!("{:#?}", pair);
                    unreachable!()
                }
            },
            |lhs: PistoletExpr, op: Pair<Rule>, rhs: PistoletExpr| match op.as_rule() {
                Rule::and => PistoletExpr::And(Box::new(lhs), Box::new(rhs)),
                Rule::or => PistoletExpr::Orb(Box::new(lhs), Box::new(rhs)),
                Rule::nand => PistoletExpr::Nand(Box::new(lhs), Box::new(rhs)),
                Rule::add => PistoletExpr::Add(Box::new(lhs), Box::new(rhs)),
                Rule::sub => PistoletExpr::Sub(Box::new(lhs), Box::new(rhs)),
                Rule::mul => PistoletExpr::Mul(Box::new(lhs), Box::new(rhs)),
                Rule::div => PistoletExpr::Div(Box::new(lhs), Box::new(rhs)),
                Rule::eq => PistoletExpr::Eq(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        )
    }

    fn parse_prog(pair: Pair<Rule>) -> PistoletAST {
        match pair.as_rule() {
            Rule::program => PistoletAST::Seq(pair.into_inner().map(parse_prog).collect()),
            Rule::TERM => PistoletAST::Seq(pair.into_inner().map(parse_prog).collect()),
            Rule::sentence => parse_prog(pair.into_inner().peek().unwrap()),
            Rule::PARA_LIST => PistoletAST::Paralist(pair.into_inner().map(parse_prog).collect()),
            Rule::LET => {
                let mut new_pair = pair.into_inner();
                PistoletAST::Let(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::VAR_NAME => x.as_str().to_string(),
                            Rule::TYPE_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::VAR_NAME => x.as_str().to_string(),
                            Rule::TYPE_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    match new_pair.peek().unwrap().as_rule() {
                        Rule::EXPR_NoTy => parse_expr(new_pair.peek().unwrap().into_inner()),
                        _ => unreachable!(),
                    },
                )
            }
            Rule::IF => {
                let mut new_pair = pair.into_inner();
                PistoletAST::If(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::BOOL_EXPR => parse_expr(x.into_inner()),
                            _ => unreachable!(),
                        }
                    },
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::TERM => Box::new(parse_prog(x)),
                            _ => unreachable!(),
                        }
                    },
                    match new_pair.peek().unwrap().as_rule() {
                        Rule::TERM => Box::new(parse_prog(new_pair.peek().unwrap())),
                        _ => unreachable!(),
                    },
                )
            }
            Rule::WHILE => {
                let mut new_pair = pair.into_inner();
                PistoletAST::While(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::BOOL_EXPR => parse_expr(x.into_inner()),
                            _ => unreachable!(),
                        }
                    },
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::TERM => Box::new(parse_prog(x)),
                            _ => unreachable!(),
                        }
                    },
                )
            }
            Rule::RETURN => PistoletAST::Return(parse_expr(pair.into_inner())),
            Rule::VAR_BIND => {
                let mut new_pair = pair.into_inner();
                PistoletAST::Varbind(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::VAR_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    match new_pair.peek().unwrap().as_rule() {
                        Rule::TYPE_NAME => new_pair.as_str().to_string(),
                        _ => unreachable!(),
                    },
                )
            }
            Rule::FUN => {
                let mut new_pair = pair.into_inner();
                PistoletAST::Fun(
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::FUN_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::PARA_LIST => Box::new(parse_prog(x)),
                            _ => unreachable!(),
                        }
                    },
                    {
                        let x = new_pair.next().unwrap();
                        match x.as_rule() {
                            Rule::TYPE_NAME => x.as_str().to_string(),
                            _ => unreachable!(),
                        }
                    },
                    match new_pair.peek().unwrap().as_rule() {
                        Rule::TERM => Box::new(parse_prog(new_pair.peek().unwrap())),
                        _ => unreachable!(),
                    },
                )
            }
            Rule::EOI => PistoletAST::EOI,
            _ => {
                println!("{:#?}", pair);
                unimplemented!()
            }
        }
    }

    Ok(parse_prog(pistolet_prog))
}
