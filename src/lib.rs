#[cfg(test)]
macro_rules! test_parse {
    ($input:expr, $fn:ident, $rule:ident, force_fail) => {{
        let result = test_parse!($input, $fn, $rule, gimme);
        dbg!(result);
        panic!("test_parse force failure");
    }};

    ($input:expr, $fn:ident, $rule:ident, gimme) => {{
        pest::set_error_detail(true);
        let mut pairs = $fn($input).unwrap();
        let return_thing = pairs.clone();
        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::$rule);

        return_thing
    }};

    ($input:expr, $fn:ident, $rule:ident) => {
        let mut pairs = $fn($input).unwrap();
        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::$rule);
    };
}
#[cfg(test)]
macro_rules! test_parse_err {
    ($input:expr, $fn:ident) => {
        assert!($fn($input).is_err());
    };
}

pub mod declarations;
pub mod members;
pub mod types;

#[derive(pest_derive::Parser)]
#[grammar = "rbs.pest"]
pub struct RbsParser;

// use pest::{error::Error, iterators::Pairs, pratt_parser::PrattParser, Parser as _};

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
