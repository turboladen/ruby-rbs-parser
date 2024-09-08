use pest::{error::Error, iterators::Pairs, Parser as _};

use crate::{RbsParser, Rule};

pub fn parse_class_instance_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::TypeClassInstanceName, source)
}

pub fn parse_interface_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::TypeInterfaceName, source)
}

pub fn parse_alias_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::TypeAliasName, source)
}

pub fn parse_singleton_class_name(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::TypeSingletonClassName, source)
}

pub fn parse_literal(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::literal, source)
}

pub fn parse_string_literal(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::LiteralString, source)
}

pub fn parse_symbol_literal(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::LiteralSymbol, source)
}

pub fn parse_integer_literal(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    RbsParser::parse(Rule::LiteralInteger, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_class_name_test() {
        assert!(parse_class_instance_name("x").is_err());

        test_parse!("X", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("X[T]", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("Foo", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("Foo[U]", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("FooBar", parse_class_instance_name, TypeClassInstanceName);
        test_parse!(
            "FooBar[Meow]",
            parse_class_instance_name,
            TypeClassInstanceName
        );
        test_parse!("FOOBAR", parse_class_instance_name, TypeClassInstanceName);
        test_parse!(
            "FOOBAR[A, B, C]",
            parse_class_instance_name,
            TypeClassInstanceName
        );
        test_parse!("Foo_Bar", parse_class_instance_name, TypeClassInstanceName);
        test_parse!(
            "Foo_Bar_BAZ",
            parse_class_instance_name,
            TypeClassInstanceName
        );
        test_parse!("::X", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("::X::Why", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("X::Y", parse_class_instance_name, TypeClassInstanceName);
        test_parse!("X123", parse_class_instance_name, TypeClassInstanceName);
    }

    #[test]
    fn parse_interface_name_test() {
        assert!(parse_interface_name("X").is_err());

        test_parse!("_H", parse_interface_name, TypeInterfaceName);
        test_parse!("_Hashing", parse_interface_name, TypeInterfaceName);
        test_parse!("_Each[A, B]", parse_interface_name, TypeInterfaceName);
        test_parse!("_Each_Iter[A, B]", parse_interface_name, TypeInterfaceName);
        test_parse!(
            "::Foo::Bar::_Each_Iter[A, B]",
            parse_interface_name,
            TypeInterfaceName
        );
    }

    #[test]
    fn parse_alias_name_test() {
        test_parse_err!("X", parse_alias_name);
        test_parse_err!("::X", parse_alias_name);
        test_parse_err!("::X::Y", parse_alias_name);
        test_parse_err!("::abcd::efgh", parse_alias_name);

        test_parse!("a", parse_alias_name, TypeAliasName);
        test_parse!("a_b_c", parse_alias_name, TypeAliasName);
        test_parse!("aBcD", parse_alias_name, TypeAliasName);
        test_parse!("::aBcD", parse_alias_name, TypeAliasName);
        test_parse!("::Foo::bar", parse_alias_name, TypeAliasName);
        test_parse!("Foo::bar", parse_alias_name, TypeAliasName);
    }

    #[test]
    fn parse_singleton_class_name_test() {
        test_parse!(
            "singleton(Foo)",
            parse_singleton_class_name,
            TypeSingletonClassName
        );
        test_parse!(
            "singleton ( Foo )",
            parse_singleton_class_name,
            TypeSingletonClassName
        );
        test_parse!(
            "singleton(::Foo)",
            parse_singleton_class_name,
            TypeSingletonClassName
        );

        test_parse!(
            "singleton(Foo::Bar)",
            parse_singleton_class_name,
            TypeSingletonClassName
        );

        // NOTE: Spec says:
        // Class singleton type cannot be parametrized.
        test_parse_err!("singleton(Foo[A])", parse_singleton_class_name);
    }

    #[test]
    fn parse_string_literal_test() {
        test_parse!(r#"''"#, parse_string_literal, LiteralString);
        test_parse!(r#"'hi'"#, parse_string_literal, LiteralString);
        test_parse!(r#"'foo \a bar'"#, parse_string_literal, LiteralString);
        test_parse!(r#"'foo \b bar'"#, parse_string_literal, LiteralString);
        test_parse!(r#"'foo \t bar'"#, parse_string_literal, LiteralString);
        test_parse!(r#"'🌮  \"  '"#, parse_string_literal, LiteralString);

        test_parse!(r#""""#, parse_string_literal, LiteralString);
        test_parse!(r#""hi""#, parse_string_literal, LiteralString);
        test_parse!(r#""foo \a bar""#, parse_string_literal, LiteralString);
        test_parse!(r#""foo \b bar""#, parse_string_literal, LiteralString);
        test_parse!(r#""foo \t bar""#, parse_string_literal, LiteralString);
        test_parse!(r#""🌮   ""#, parse_string_literal, LiteralString);
    }

    #[test]
    fn parse_symbol_literal_test() {
        test_parse!(":hi", parse_symbol_literal, LiteralSymbol);
        test_parse!(":foo_bar", parse_symbol_literal, LiteralSymbol);
        test_parse!(":🌮", parse_symbol_literal, LiteralSymbol);

        test_parse!(r#":''"#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":'hi'"#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":'foo \a bar'"#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":'foo \b bar'"#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":'foo \t bar'"#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":'🌮  \"  '"#, parse_symbol_literal, LiteralSymbol);

        test_parse!(r#":"""#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":"hi""#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":"foo \a bar""#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":"foo \b bar""#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":"foo \t bar""#, parse_symbol_literal, LiteralSymbol);
        test_parse!(r#":"🌮   ""#, parse_symbol_literal, LiteralSymbol);
    }

    #[test]
    fn parse_integer_literal_test() {
        test_parse!("0", parse_integer_literal, LiteralInteger);
        test_parse!("1_2_3", parse_integer_literal, LiteralInteger);

        test_parse!("0d0", parse_integer_literal, LiteralInteger);
        test_parse!("0D0", parse_integer_literal, LiteralInteger);

        test_parse!("0x0", parse_integer_literal, LiteralInteger);
        test_parse!("0X0", parse_integer_literal, LiteralInteger);

        test_parse!("00", parse_integer_literal, LiteralInteger);
        test_parse!("0o0", parse_integer_literal, LiteralInteger);
        test_parse!("0o01234567", parse_integer_literal, LiteralInteger);
        test_parse_err!("0o0123456789", parse_integer_literal);

        test_parse!("0b0", parse_integer_literal, LiteralInteger);
        test_parse!("0B0", parse_integer_literal, LiteralInteger);
        test_parse!("0b010_101", parse_integer_literal, LiteralInteger);
    }
}
