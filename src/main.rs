extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pistolet.pest"]
pub struct PistoletParser;

fn main() {
    let program = 
    
    "{
        let y = 1.
        if true { let x = 1 + y. } {let x = 2. }.
    }"
    ;
    let parsed_program = PistoletParser::parse(Rule::Program, program);
    println!("{:#?}", parsed_program);
}