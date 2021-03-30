include!("parser.rs");
include!("interpreter.rs");

use colored::*;
use std::fs;
use std::thread;

fn main() {
    println!("{}", "[Test] Parser Test Begin!".green());
    let file_path = "src/test/test_parser.pst";
    match parse_to_ast(&fs::read_to_string(file_path).expect("cannot read file")) {
        Ok(_) => println!("{}", "[Test] Parser Test Passed!".green()),
        Err(_) => {
            println!("{}", "[Failed] Parser Test Failed!".red());
            return;
        }
    }
    println!("{}", "[Test] Interpreter Test Begin!".green());
    thread::Builder::new().stack_size(1024000 * 1024).spawn(move || {
        let file_path = "src/test/test_eval.pst";
        let eval_result;
        let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");
        let state = ProgStates::new();
        eval_result = ast_eval(parse_to_ast(&unparsed_file).unwrap(), state);

        match eval_result {
            Ok(result) => result.print(),
            Err(err_code) => match err_code {
                RuntimeErr::ReturnValue(expr_value) => println!(
                    "[Return] Exit with {} : {}",
                    expr_value.get_type(),
                    expr_value.get_value()
                ),
                _ => err_code.print(),
            },
        }
    }).unwrap().join().expect("Couldn't join on the associated thread");
    println!("{}", "[Test] Interpreter Test Passed!".green());
}
