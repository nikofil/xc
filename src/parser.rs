use crate::error::{Error, Result};
use crate::reprs::parse_num;
use std::fmt::Display;

#[derive(Eq, PartialEq, Debug)]
pub enum Operator {
    Sentinel,
    Add,
    Mul,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Sentinel => "~",
            Operator::Add => "+",
            Operator::Mul => "*",
        })
    }
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

    fn push_expr(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) -> Result<()> {
        let right = operands.pop();
        let left = operands.pop();
        let last_oper = operators.pop().unwrap();
        if let (Some(left), Some(right)) = (left, right) {
            operands.push(Operand::Term(last_oper, Box::new(left), Box::new(right)));
            Ok(())
        } else {
            Err(Error::ExprParseError(last_oper))
        }
    }
}

impl<'a> Into<Result<Operand>> for Parser<'a> {
    fn into(self) -> Result<Operand> {
        let mut operands: Vec<Operand> = Vec::new();
        let mut operators: Vec<Operator> = vec![Operator::Sentinel];
        for i in self {
            match i? {
                Term::Num(num) => operands.push(Operand::Num(num)),
                Term::Operator(oper) => {
                    while Self::op_precedence(&oper) <= Self::op_precedence(&operators.last().unwrap()) {
                        Self::push_expr(&mut operands, &mut operators)?;
                    }
                    operators.push(oper);
                },
            }
        }
        while operators.len() > 1 {
            Self::push_expr(&mut operands, &mut operators)?;
        }
        let mut operand_iter = operands.into_iter();
        let res = operand_iter.next().ok_or(Error::ExprTermsError)?;
        if operand_iter.next().is_some() {
            Err(Error::ExprTermsError)
        } else {
            Ok(res)
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
fn test_lexer() {
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

#[test]
fn test_parser_simple() {
    let oper: Operand = Result::from(Parser::new("1").into()).unwrap();
    assert_eq!(oper, Operand::Num(1));

    let oper: Operand = Result::from(Parser::new("1 * 2").into()).unwrap();
    assert_eq!(oper, Operand::Term(
        Operator::Mul,
        Box::new(Operand::Num(1)),
        Box::new(Operand::Num(2))
    ));

    let oper: Operand = Result::from(Parser::new("0*1+2 * 0x10").into()).unwrap();
    assert_eq!(oper, Operand::Term(
        Operator::Add,
        Box::new(Operand::Term(
            Operator::Mul,
            Box::new(Operand::Num(0)),
            Box::new(Operand::Num(1)),
        )),
        Box::new(Operand::Term(
            Operator::Mul,
            Box::new(Operand::Num(2)),
            Box::new(Operand::Num(16)),
        ))
    ));
}

#[test]
fn test_parser_errs() {
    assert!(Result::from(Parser::new("0*").into()).is_err());
    assert!(Result::from(Parser::new("1 2").into()).is_err());
    assert!(Result::from(Parser::new("   ").into()).is_err());
    assert!(Result::from(Parser::new("1*2*+3").into()).is_err());
    assert!(Result::from(Parser::new("1*2*").into()).is_err());
}
