use crate::error::{Error, Result};
use crate::reprs::parse_num;

#[derive(Eq, PartialEq, Debug)]
enum Operator {
    Sentinel,
    Add,
    Mul,
}

#[derive(Eq, PartialEq, Debug)]
enum Operand {
    Num(i64),
    Term(Operator, Box<Operand>, Box<Operand>),
}

#[derive(Eq, PartialEq, Debug)]
enum Term {
    Operator(Operator),
    Operand(Operand),
    EOF,
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

    fn next_term(&mut self) -> Result<Term> {
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
                parse_num(token).map(|num| Term::Operand(Operand::Num(num)))
            } else {
                let token = self.take_input_until(|nc| nc.is_alphanumeric() || nc.is_whitespace());
                match token {
                    "+" => Ok(Term::Operator(Operator::Add)),
                    "*" => Ok(Term::Operator(Operator::Mul)),
                    _ => Err(Error::OperatorParseError(token)),
                }
            }
        } else {
            Ok(Term::EOF)
        }
    }
}

#[test]
fn test_parser_simple() {
    let mut parser = Parser::new("111 + 222");
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(111)));
    assert_eq!(parser.next_term().unwrap(), Term::Operator(Operator::Add));
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(222)));
    assert_eq!(parser.next_term().unwrap(), Term::EOF);

    let mut parser = Parser::new("11*22+33");
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(11)));
    assert_eq!(parser.next_term().unwrap(), Term::Operator(Operator::Mul));
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(22)));
    assert_eq!(parser.next_term().unwrap(), Term::Operator(Operator::Add));
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(33)));
    assert_eq!(parser.next_term().unwrap(), Term::EOF);

    let mut parser = Parser::new("11 ** 22");
    assert_eq!(parser.next_term().unwrap(), Term::Operand(Operand::Num(11)));
    assert!(parser.next_term().is_err());
}
