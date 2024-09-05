use pest::Parser as _;
use ruby_rbs_parser::{RbsParser, Rule};

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let output = RbsParser::parse(Rule::rtype, &src);
    println!("{:#?}", output);

    let output = ruby_rbs_parser::pest_parser::parse_class(&src);
    println!("{:#?}", output);
}
