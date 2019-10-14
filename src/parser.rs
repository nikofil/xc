use crate::error::{Error, Result};
use crate::reprs::parse_num;

#[derive(Eq, PartialEq, Debug)]
pub enum Operator {
    Sentinel,
    Add,
    Mul,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Operand {
    Num(i64),
    Term(Operator, Box<Operand>, Box<Operand>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Term {
    Operator(Operator),
    Num(i64),
}

pub struct Parser<'a> {
    input: &'a str,
    operands: Vec<Operand>,
    operators: Vec<Operator>,
}

impl Parser<'_> {
    fn op_precedence(op: &Operator) -> u32 {
        match op {
            Operator::Sentinel => 0,
            Operator::Add => 20,
            Operator::Mul => 40,
        }
    }
    pub fn new(input: &str) -> Parser {
        Parser {
            input,
            operands: Vec::new(),
            operators: vec![Operator::Sentinel],
        }
    }

    fn take_input_until<P>(&mut self, predicate: P) -> &str
        where  P: FnMut(char) -> bool
    {
        let end = self.input.chars().position(predicate);
        if let Some(end) = end {
            let cur_str = self.input;
            self.input = &self.input[end..];
            &cur_str[..end]
        } else {
            let cur_str = self.input;
            self.input = "";
            cur_str
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Term>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.input.chars().next() {
            if c.is_whitespace() {
                self.input = &self.input[1..];
            } else {
                break;
            }
        }
        if let Some(c) = self.input.chars().next() {
            if c.is_alphanumeric() {
                let token = self.take_input_until(|nc| !nc.is_alphanumeric());
                Some(parse_num(token).map(|num| Term::Num(num)))
            } else {
                let token = self.take_input_until(|nc| nc.is_alphanumeric() || nc.is_whitespace());
                match token {
                    "+" => Some(Ok(Term::Operator(Operator::Add))),
                    "*" => Some(Ok(Term::Operator(Operator::Mul))),
                    _ => Some(Err(Error::OperatorParseError(token.to_string()))),
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn test_parser_simple() {
    let mut parser = Parser::new("111 + 222");
    let terms = parser.map(|t| t.unwrap()).collect::<Vec<Term>>();
    assert_eq!(terms, vec![
        Term::Num(111),
        Term::Operator(Operator::Add),
        Term::Num(222)
    ]);

    let mut parser = Parser::new("11*22+33");
    let terms = parser.map(|t| t.unwrap()).collect::<Vec<Term>>();
    assert_eq!(terms, vec![
        Term::Num(11),
        Term::Operator(Operator::Mul),
        Term::Num(22),
        Term::Operator(Operator::Add),
        Term::Num(33)
    ]);

    let mut parser = Parser::new("11 ** 22");
    assert_eq!(parser.next().unwrap().unwrap(), Term::Num(11));
    assert!(parser.next().unwrap().is_err());
}
