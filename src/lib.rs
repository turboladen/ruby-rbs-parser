#[derive(pest_derive::Parser)]
#[grammar = "rbs.pest"]
pub struct RbsParser;

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

pub fn parse_class_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::class_decl, source)
}

pub fn parse_module_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::module_decl, source)
}

pub fn parse_const_decl(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::const_decl, source)
}

pub fn parse_method_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::method_member, source)
}

pub fn parse_alias_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::alias_member, source)
}

pub fn parse_attribute_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::attribute_member, source)
}

pub fn parse_include_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::include_member, source)
}

pub fn parse_extend_member(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    crate::RbsParser::parse(Rule::extend_member, source)
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
