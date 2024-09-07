use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_class_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::class_name, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_name_test() {
        let text = "X";
        let _ = parse_class_name(text).unwrap();

        let text = "Foo";
        let _ = parse_class_name(text).unwrap();

        let text = "FooBar";
        let _ = parse_class_name(text).unwrap();

        let text = "FOOBAR";
        let _ = parse_class_name(text).unwrap();

        let text = "Foo_Bar";
        let _ = parse_class_name(text).unwrap();

        let text = "Foo_Bar_BAZ";
        let _ = parse_class_name(text).unwrap();

        let text = "::X";
        let _ = parse_class_name(text).unwrap();

        let text = "::X::Why";
        let _ = parse_class_name(text).unwrap();

        let text = "X::Y";
        let _ = parse_class_name(text).unwrap();

        let text = "X123";
        let _ = parse_class_name(text).unwrap();
    }
}
