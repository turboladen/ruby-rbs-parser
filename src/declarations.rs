use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_class_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::class_decl, source)
}

pub fn parse_module_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::module_decl, source)
}

pub fn parse_const_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::const_decl, source)
}

// NOTE: FYI: https://github.com/ruby/rbs/blob/master/test/rbs/parser_test.rb
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_decl_no_members_positive_test() {
        let text = "class Foo\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[T, U]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[out T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[in T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[unchecked T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[unchecked out T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[unchecked in T]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[T < _Output]\nend";
        let _ = parse_class_decl(text).unwrap();

        let text = "class Foo[T] < Bar[T]\nend";
        let _ = parse_class_decl(text).unwrap();
    }

    #[test]
    fn module_decl_no_members_positive_test() {
        let text = "module Foo\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[T, U]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[out T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[in T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[unchecked T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[unchecked out T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[unchecked in T]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[T < _Output]\nend";
        let _ = parse_module_decl(text).unwrap();

        let text = "module Foo[T] : String, _Array[Symbol]\nend";
        let _ = parse_module_decl(text).unwrap();
    }

    #[test]
    fn const_decl_test() {
        let text = "Person: String";
        let _ = parse_const_decl(text).unwrap();

        let text = "Person::DefaultEmailAddress: String";
        let _ = parse_const_decl(text).unwrap();
    }
}
