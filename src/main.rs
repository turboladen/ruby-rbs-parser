use pest::Parser as _;
use rbsby::{RbsParser, Rule};

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    // println!("{}", src);
    // println!("{}", chum::parser().parse(src));
    let output = RbsParser::parse(Rule::rtype, &src);
    println!("{:#?}", output);

    let output = rbsby::pest_parser::parse_class(&src);
    println!("{:#?}", output);
}
