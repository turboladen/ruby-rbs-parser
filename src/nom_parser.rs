use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult, Parser as _};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    True,
    False,
}

pub struct Context<'input> {
    current_offset: usize,
    end_offset: usize,
    text: &'input str,
}

impl<'input> Context<'input> {
    pub fn new(text: &'input str) -> Self {
        Self {
            current_offset: 0,
            end_offset: text.len(),
            text,
        }
    }
}

pub fn parse(input: &str) -> IResult<&str, Node> {
    let mut context = Context::new(input);
    parse_literal(input, &mut context)
}

fn parse_literal<'a>(input: &'a str, context: &mut Context) -> IResult<&'a str, Node> {
    parse_boolean(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Node> {
    let parse_true = value(Node::True, tag("true"));
    let parse_false = value(Node::False, tag("false"));

    alt((parse_true, parse_false)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_false_test() {
        assert_eq!(parse_boolean("true").unwrap(), ("", Node::True));
        assert_eq!(
            parse_boolean("true  false").unwrap(),
            ("  false", Node::True)
        );

        assert_eq!(parse_literal("true").unwrap(), ("", Node::True));
        assert_eq!(
            parse_literal("true  false").unwrap(),
            ("  false", Node::True)
        );
        assert_eq!(parse_literal("truefalse").unwrap(), ("false", Node::True));
    }
}
