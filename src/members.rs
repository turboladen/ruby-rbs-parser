use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_method_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::method_member, source)
}

pub fn parse_alias_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::alias_member, source)
}

pub fn parse_attribute_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::attribute_member, source)
}

pub fn parse_include_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::include_member, source)
}

pub fn parse_extend_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::extend_member, source)
}

// NOTE: FYI: https://github.com/ruby/rbs/blob/master/test/rbs/parser_test.rb
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_member_test() {
        let text = "def foo: () -> A";
        let _ = parse_method_member(text).unwrap();

        let text = "def     foo    : ()     ->     A";
        let _ = parse_method_member(text).unwrap();

        let text = "def self.bar: () -> A";
        let _ = parse_method_member(text).unwrap();

        let text = "def      self.   bar: () -> A";
        let _ = parse_method_member(text).unwrap();

        let text = "def      self   .bar   : () -> A";
        let _ = parse_method_member(text).unwrap();

        let text = "def      self .: () -> A";
        assert!(parse_method_member(text).is_err());

        let text = "def self?.baz: () -> A";
        let _ = parse_method_member(text).unwrap();

        let text = "def self ? . baz :()->A";
        let _ = parse_method_member(text).unwrap();

        let text = "def self?.baz:()->A";
        let _ = parse_method_member(text).unwrap();
    }

    #[test]
    fn alias_member_test() {
        let text = "alias hello world";
        let _ = parse_alias_member(text).unwrap();

        let text = "alias self.hello self.world";
        let _ = parse_alias_member(text).unwrap();

        let text = "alias    self   .   hello    self   .   world";
        let _ = parse_alias_member(text).unwrap();
    }

    #[test]
    fn attribute_member_test() {
        let text = "attr_reader string: String";
        let _ = parse_attribute_member(text).unwrap();

        let text = "attr_writer name (): Integer";
        let _ = parse_attribute_member(text).unwrap();

        let text = "attr_writer name (@raw_name): String";
        let _ = parse_attribute_member(text).unwrap();

        let text = "attr_accessor people (): Array[Person]";
        let _ = parse_attribute_member(text).unwrap();
    }

    #[test]
    fn include_member_test() {
        let text = "include FooBar";
        let _ = parse_include_member(text).unwrap();

        let text = "include X[A]";
        let _ = parse_include_member(text).unwrap();

        let text = "include Array[A]";
        let _ = parse_include_member(text).unwrap();

        let text = "include _ToS";
        let _ = parse_include_member(text).unwrap();

        let text = "include _Each[T]";
        let _ = parse_include_member(text).unwrap();
    }

    #[test]
    fn extend_member_test() {
        let text = "extend FooBar";
        let _ = parse_extend_member(text).unwrap();

        let text = "extend X[A]";
        let _ = parse_extend_member(text).unwrap();

        let text = "extend Array[A]";
        let _ = parse_extend_member(text).unwrap();

        let text = "extend _ToS";
        let _ = parse_extend_member(text).unwrap();

        let text = "extend _Each[T]";
        let _ = parse_extend_member(text).unwrap();
    }
}
