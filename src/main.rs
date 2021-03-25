extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pistolet.pest"]
pub struct PistoletParser;

fn main() {
    let fun_program = 
    "fun foo (X: NAT)(Y : BOOL) -> BOOL 
    {
        let x: bool = true.
        return x.
    }.
    
    let y: nat = 2 + 10 + 1 * 1 / 2 - 3.
    let b: bool = true && false || true. 
    "
    ;
    let fun_program_parsed = PistoletParser::parse(Rule::Program, fun_program);
    println!("{:#?}", fun_program_parsed);

}