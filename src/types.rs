use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_class_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::class_name, source)
}

pub fn parse_interface_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::interface_name, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_name_test() {
        test_parse!("X", parse_class_name, class_name);
        test_parse!("X[T]", parse_class_name, class_name);
        test_parse!("Foo", parse_class_name, class_name);
        test_parse!("Foo[U]", parse_class_name, class_name);
        test_parse!("FooBar", parse_class_name, class_name);
        test_parse!("FooBar[Meow]", parse_class_name, class_name);
        test_parse!("FOOBAR", parse_class_name, class_name);
        test_parse!("FOOBAR[A, B, C]", parse_class_name, class_name);
        test_parse!("Foo_Bar", parse_class_name, class_name);
        test_parse!("Foo_Bar_BAZ", parse_class_name, class_name);
        test_parse!("::X", parse_class_name, class_name);
        test_parse!("::X::Why", parse_class_name, class_name);
        test_parse!("X::Y", parse_class_name, class_name);
        test_parse!("X123", parse_class_name, class_name);
    }

    #[test]
    fn interface_name_test() {
        let text = "X";
        assert!(parse_interface_name(text).is_err());

        test_parse!("_H", parse_interface_name, interface_name);
        test_parse!("_Hashing", parse_interface_name, interface_name);
        test_parse!("_Each[A, B]", parse_interface_name, interface_name);
        test_parse!("_Each_Iter[A, B]", parse_interface_name, interface_name);
    }
}
