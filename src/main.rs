include!("parser.rs");
include!("interpreter.rs");

use std::fs;

fn main() {
    let file_path = "src/test/test_parser.pst";
    let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");
    println!("{:#?}", parse_to_ast(&unparsed_file));
    let file_path = "src/test/test_eval.pst";
    let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");
    let state = ProgState(Rc::new(RefCell::new(ProgList {
        var_list: HashMap::new(),
        func_list: HashMap::new(),
    })));
    println!(
        "{:#?}",
        ast_eval(parse_to_ast(&unparsed_file).unwrap(), &state)
    );
}
