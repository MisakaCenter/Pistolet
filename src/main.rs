include!("parser.rs");

use std::fs;

fn main() {
    let file_path = "src/test/test_parser.pst";
    let unparsed_file = fs::read_to_string(file_path)
                        .expect("cannot read file");
    println!("{:#?}", parse_to_ast(&unparsed_file));
}