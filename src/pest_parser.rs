use crate::Rule;
// use pest::{error::Error, iterators::Pairs, pratt_parser::PrattParser, Parser as _};
use pest::{error::Error, iterators::Pairs, Parser as _};

// lazy_static::lazy_static! {
//     static ref PRATT_PARSER: PrattParser<Rule> = {
//         use pest::pratt_parser::{Assoc::{Left, Right}, Op};
//         // use Assoc::*;
//         // use Rule::*;
//
//         PrattParser::new()
//             .op(Op::postfix(Rule::namespace_op))
//     };
// }
//
pub fn parse_class(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let output = crate::RbsParser::parse(Rule::class_decl, source);

    // PRATT_PARSER::parse(Rule::class_decl, &src)
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_decl_no_members_positive_test() {
        let text = "class Foo\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[T, U]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[out T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[in T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[unchecked T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[unchecked out T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[unchecked in T]\nend";
        let _ = parse_class(text).unwrap();

        let text = "class Foo[T < _Output]\nend";
        let x = parse_class(text).unwrap();

        dbg!(x);
    }
}
