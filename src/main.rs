include!("parser.rs");
include!("interpreter.rs");

use std::fs;

fn main() {
    println!("[Test] Parser Test Begin!");
    let file_path = "src/test/test_parser.pst";
    match parse_to_ast(&fs::read_to_string(file_path).expect("cannot read file")) {
        Ok(_) => println!("[Test] Parser Test Complete!"),
        Err(_) => {
            println!("[Failed] Parser Test Failed!");
            return;
        }
    }
    println!("[Test] Interpreter Test Begin!");
    let file_path = "src/test/test_eval.pst";
    let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");

    let state = ProgStates::new();    
    let eval_result = ast_eval(parse_to_ast(&unparsed_file).unwrap(), state);
    match eval_result {
        Ok(result) => result.print(),
        Err(err_code) => err_code.print(),
    }
    println!("[Test] Interpreter Test Complete!");
}
