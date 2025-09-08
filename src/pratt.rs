use std::collections::HashMap;
use std::sync::LazyLock;

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
// use once_cell::sync::Lazy;
use pest::pratt_parser::{Assoc, Op, PrattParser};

use crate::types::{parse_integer_literal, parse_string_literal, parse_symbol_literal};
use crate::Rule;
// use crate::Rule::*;

// use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Basic types
    Self_,
    Instance,
    Class,
    Bool,
    Boolish,
    Untyped,
    Nil,
    Top,
    Bot,
    Void,

    // Compound types
    Union(Box<Type>, Box<Type>),
    Intersection(Box<Type>, Box<Type>),
    Optional(Box<Type>),

    // Class and interface types
    ClassInstance {
        name: String,
        namespace: Vec<String>,
        type_arguments: Vec<Type>,
    },
    Interface {
        name: String,
        namespace: Vec<String>,
        type_arguments: Vec<Type>,
    },
    Alias {
        name: String,
        namespace: Vec<String>,
        type_arguments: Vec<Type>,
    },
    ClassSingleton {
        name: String,
        namespace: Vec<String>,
    },

    // Record and tuple types
    Record(HashMap<String, Type>),
    Tuple(Vec<Type>),

    // Literals
    StringLit(String),
    SymbolLit(String),
    IntegerLit(i64),
    TrueLit,
    FalseLit,

    // Type variables
    TypeVariable(String),

    // Proc types
    Proc {
        parameters: Option<Parameters>,
        self_type: Option<Box<Type>>,
        block: Option<Block>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    pub required: Vec<Parameter>,
    pub optional: Vec<Parameter>,
    pub rest: Option<Parameter>,
    pub trailing: Vec<Parameter>,
    pub kwargs: Option<KwArgs>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub type_: Box<Type>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KwArgs {
    Rest(Parameter),
    Required {
        name: String,
        type_: Box<Type>,
        rest: Box<KwArgs>,
    },
    Optional {
        name: String,
        type_: Box<Type>,
        rest: Box<KwArgs>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub parameters: Option<Parameters>,
    pub self_type: Option<Box<Type>>,
    pub return_type: Box<Type>,
    pub optional: bool,
}

/// Parser for RBS type expressions, handling operator precedence.
/// Precedence order (lowest to highest):
/// 1. Union (|)
/// 2. Intersection (&)
/// 3. Optional (?)
static TYPE_PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::*;

    PrattParser::new()
        // Union has lowest precedence
        .op(Op::infix(Rule::union, Left))
        // Intersection binds tighter than union
        .op(Op::infix(Rule::intersection, Left))
        // Optional has highest precedence
        .op(Op::postfix(Rule::optional))
});

fn parse_alias_name_type(pair: Pair<Rule>) -> Type {
    todo!("parse_alias_name_type")
}

fn parse_class_singleton_name_type(pair: Pair<Rule>) -> Type {
    todo!("parse_class_singleton_name_type")
}

fn parse_type_variable(pair: Pair<Rule>) -> Type {
    todo!("parse_type_variable")
}

/// Type expression parser that uses the Pratt parser to handle operator precedence
pub fn parse_type_expr(pairs: Pairs<Rule>) -> Result<Type, Error> {
    TYPE_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::class_instance_name_type => parse_class_instance_name_type(primary),
            Rule::interface_name_type => parse_interface_name_type(primary),
            Rule::alias_name_type => parse_alias_name_type(primary),
            Rule::class_singleton_name_type => parse_class_singleton_name_type(primary),
            Rule::literal => parse_literal(primary),
            Rule::record_type => parse_record_type(primary),
            Rule::tuple_type => parse_tuple_type(primary),
            Rule::type_variable => parse_type_variable(primary),
            Rule::proc_type => parse_proc_type(primary),
            token => match token.as_str() {
                "self" => Type::Self_,
                "instance" => Type::Instance,
                "class" => Type::Class,
                "bool" => Type::Bool,
                "boolish" => Type::Boolish,
                "untyped" => Type::Untyped,
                "nil" => Type::Nil,
                "top" => Type::Top,
                "bot" => Type::Bot,
                "void" => Type::Void,
                _ => unreachable!("Unexpected primary type token: {:?}", token),
            },
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::union => Type::Union(Box::new(lhs), Box::new(rhs)),
            Rule::intersection => Type::Intersection(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!("Unexpected infix operator: {:?}", op),
        })
        .map_postfix(|expr, op| match op.as_rule() {
            Rule::optional => Type::Optional(Box::new(expr)),
            _ => unreachable!("Unexpected postfix operator: {:?}", op),
        })
        .parse(pairs)
}

fn parse_class_instance_name_type(pair: Pair<Rule>) -> Type {
    let mut inner = pair.into_inner();

    // Parse class name and namespace
    let class_name_pair = inner.next().unwrap();
    let (name, namespace) = parse_namespaced_name(class_name_pair);

    // Parse type arguments if they exist
    let type_arguments = inner
        .next() // type_arguments is optional
        .map(|type_args| {
            type_args
                .into_inner()
                .map(|type_arg| parse_type_expr(type_arg.into_inner()).unwrap())
                .collect()
        })
        .unwrap_or_default();

    Type::ClassInstance {
        name,
        namespace,
        type_arguments,
    }
}

fn parse_interface_name_type(pair: Pair<Rule>) -> Type {
    let mut inner = pair.into_inner();

    // Parse interface name and namespace
    let interface_name_pair = inner.next().unwrap();
    let (name, namespace) = parse_namespaced_name(interface_name_pair);

    // Parse type arguments if they exist
    let type_arguments = inner
        .next()
        .map(|type_args| {
            type_args
                .into_inner()
                .map(|type_arg| parse_type_expr(type_arg.into_inner()).unwrap())
                .collect()
        })
        .unwrap_or_default();

    Type::Interface {
        name,
        namespace,
        type_arguments,
    }
}

fn parse_record_type(pair: Pair<Rule>) -> Type {
    let mut fields = HashMap::new();

    // Iterate through record entries
    for record_entry in pair.into_inner() {
        let mut entry_parts = record_entry.into_inner();
        let name = entry_parts.next().unwrap().as_str().to_string();
        let type_ = parse_type_expr(entry_parts.next().unwrap().into_inner()).unwrap();
        fields.insert(name, type_);
    }

    Type::Record(fields)
}

fn parse_tuple_type(pair: Pair<Rule>) -> Type {
    let types = pair
        .into_inner()
        .map(|type_pair| parse_type_expr(type_pair.into_inner()).unwrap())
        .collect();

    Type::Tuple(types)
}

fn parse_literal(pair: Pair<Rule>) -> Type {
    let literal = pair.into_inner().next().unwrap();
    match literal.as_rule() {
        Rule::string_literal => Type::StringLit(parse_string_literal(literal)),
        Rule::symbol_literal => Type::SymbolLit(parse_symbol_literal(literal)),
        Rule::integer_literal => Type::IntegerLit(parse_integer_literal(literal)),
        _ => match literal.as_str() {
            "true" => Type::TrueLit,
            "false" => Type::FalseLit,
            _ => unreachable!("Unknown literal type: {:?}", literal),
        },
    }
}

fn parse_proc_type(pair: Pair<Rule>) -> Type {
    let mut inner = pair.into_inner();

    // Skip the '^' token
    let mut next = inner.next().unwrap();

    // Parse parameters if present
    let parameters = if next.as_rule() == Rule::parameters {
        let params = parse_parameters(next);
        next = inner.next().unwrap();
        Some(params)
    } else {
        None
    };

    // Parse self type binding if present
    let self_type = if next.as_rule() == Rule::self_type_binding {
        let self_type = parse_self_type_binding(next);
        next = inner.next().unwrap();
        Some(self_type)
    } else {
        None
    };

    // Parse block if present
    let block = if next.as_rule() == Rule::block {
        let block = parse_block(next);
        next = inner.next().unwrap();
        Some(block)
    } else {
        None
    };

    // Parse return type
    let return_type = parse_type_expr(next.into_inner()).unwrap();

    Type::Proc {
        parameters,
        self_type: self_type.map(Box::new),
        block,
        return_type: Box::new(return_type),
    }
}

// Helper functions

fn parse_namespaced_name(pair: Pair<Rule>) -> (String, Vec<String>) {
    let mut parts: Vec<String> = pair.as_str().split("::").map(|s| s.to_string()).collect();

    let name = parts.pop().unwrap();
    let namespace = parts;

    (name, namespace)
}

fn parse_parameters(pair: Pair<Rule>) -> Parameters {
    let mut required = Vec::new();
    let mut optional = Vec::new();
    let mut rest = None;
    let mut trailing = Vec::new();
    let mut kwargs = None;

    for section in pair.into_inner() {
        match section.as_rule() {
            Rule::required_positionals => {
                required = section.into_inner().map(parse_parameter).collect();
            }
            Rule::optional_positionals => {
                optional = section.into_inner().map(parse_parameter).collect();
            }
            Rule::rest_positional => {
                rest = Some(parse_parameter(section.into_inner().next().unwrap()));
            }
            Rule::trailing_positionals => {
                trailing = section.into_inner().map(parse_parameter).collect();
            }
            Rule::kwargs => {
                kwargs = Some(parse_kwargs(section));
            }
            _ => unreachable!("Unknown parameter section: {:?}", section),
        }
    }

    Parameters {
        required,
        optional,
        rest,
        trailing,
        kwargs,
    }
}

// Additional helper function implementations would go here...
