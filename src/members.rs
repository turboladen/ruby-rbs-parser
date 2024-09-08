use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_method(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::MemberMethod, source)
}

pub fn parse_alias(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::MemberAlias, source)
}

pub fn parse_attribute(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::MemberAttribute, source)
}

pub fn parse_include(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::MemberInclude, source)
}

pub fn parse_extend(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::MemberExtend, source)
}

// NOTE: FYI: https://github.com/ruby/rbs/blob/master/test/rbs/parser_test.rb
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_method_test() {
        test_parse!("def foo: () -> A", parse_method, MemberMethod);
        test_parse!(
            "def     foo    : ()     ->     A",
            parse_method,
            MemberMethod
        );
        test_parse!("def self.bar: () -> A", parse_method, MemberMethod);
        test_parse!("def      self.   bar: () -> A", parse_method, MemberMethod);
        test_parse!(
            "def      self   .bar   : () -> A",
            parse_method,
            MemberMethod
        );
        test_parse!("def self?.baz: () -> A", parse_method, MemberMethod);
        test_parse!("def self ? . baz :()->A", parse_method, MemberMethod);
        test_parse!("def self?.baz:()->A", parse_method, MemberMethod);

        test_parse_err!("def      self .: () -> A", parse_method);
    }

    #[test]
    fn parse_alias_test() {
        test_parse!("alias hello world", parse_alias, MemberAlias);
        test_parse!("alias self.hello self.world", parse_alias, MemberAlias);
        test_parse!(
            "alias    self   .   hello    self   .   world",
            parse_alias,
            MemberAlias
        );
    }

    #[test]
    fn parse_attribute_test() {
        test_parse!(
            "attr_reader string: String",
            parse_attribute,
            MemberAttribute
        );
        test_parse!(
            "attr_writer name (): Integer",
            parse_attribute,
            MemberAttribute
        );
        test_parse!(
            "attr_writer name (@raw_name): String",
            parse_attribute,
            MemberAttribute
        );
        test_parse!(
            "attr_accessor people (): Array[Person]",
            parse_attribute,
            MemberAttribute
        );
    }

    #[test]
    fn parse_include_test() {
        test_parse!("include FooBar", parse_include, MemberInclude);
        test_parse!("include X[A]", parse_include, MemberInclude);
        test_parse!("include Array[A]", parse_include, MemberInclude);
        test_parse!("include _ToS", parse_include, MemberInclude);
        test_parse!("include _Each[T]", parse_include, MemberInclude);
    }

    #[test]
    fn parse_extend_test() {
        test_parse!("extend FooBar", parse_extend, MemberExtend);
        test_parse!("extend X[A]", parse_extend, MemberExtend);
        test_parse!("extend Array[A]", parse_extend, MemberExtend);
        test_parse!("extend _ToS", parse_extend, MemberExtend);
        test_parse!("extend _Each[T]", parse_extend, MemberExtend);
    }
}
