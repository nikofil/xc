use crate::error::{Error, Result};
use crate::reprs::parse_num;
use std::fmt::Display;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Operator {
    Sentinel,
    Add,
    Sub,
    Mul,
    Div,
    Remainder,
    Pow,
    Neg,
    BNot,
    BXor,
    BOr,
    BAnd,
    LShift,
    RShift,
    Lparen,
    Rparen,
    Assign,
    FnBody,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Sentinel => panic!("Why are we displaying the sentinel?"),
                Operator::Add => "+",
                Operator::Mul => "*",
                Operator::Sub => "-",
                Operator::Div => "/",
                Operator::Remainder => "%",
                Operator::Pow => "**",
                Operator::Neg => "-",
                Operator::BNot => "~",
                Operator::BXor => "^",
                Operator::BOr => "|",
                Operator::BAnd => "&",
                Operator::LShift => "<<",
                Operator::RShift => ">>",
                Operator::Lparen => "(",
                Operator::Rparen => ")",
                Operator::Assign => "=",
                Operator::FnBody => "->",
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Operand {
    Num(i128),
    Term(Operator, Box<Operand>, Box<Operand>),
    Var(String),
    FnArgs(Vec<String>),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Num(i) => write!(f, "{}", i),
            Operand::Term(oper, lhs, rhs) => write!(f, "({} {} {})", lhs, oper, rhs),
            Operand::Var(name) => write!(f, "{}", name),
            Operand::FnArgs(args) => write!(f, "{:?}", args),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Term {
    Operator(Operator),
    Num(i128),
    Var(String),
    FnArgs(Vec<String>),
    Lparen,
    Rparen,
}

pub struct Parser<'a> {
    input: &'a str,
    last_was_operand: bool,
}

const UNARY: i32 = 100_000;

impl Parser<'_> {
    fn op_precedence(op: &Operator) -> i32 {
        match op {
            Operator::Sentinel => -1,
            Operator::Assign => 0,
            Operator::Lparen => 2,
            Operator::Rparen => 2,
            Operator::FnBody => 5,
            Operator::BOr => 10,
            Operator::BXor => 20,
            Operator::BAnd => 30,
            Operator::LShift => 40,
            Operator::RShift => 40,
            Operator::Add => 50,
            Operator::Sub => 50,
            Operator::Mul => 60,
            Operator::Div => 60,
            Operator::Remainder => 60,
            Operator::Pow => 70,
            Operator::BNot => UNARY,
            Operator::Neg => UNARY,
        }
    }
    pub fn new(input: &str) -> Parser {
        Parser {
            input,
            last_was_operand: false,
        }
    }

    fn take_input_until<P>(&mut self, predicate: P) -> &str
    where
        P: FnMut(char) -> bool,
    {
        let end = self
            .input
            .chars()
            .skip(1)
            .position(predicate)
            .map(|e| e + 1);
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
                Term::Var(var_name) => operands.push(Operand::Var(var_name)),
                Term::Lparen => {
                    operators.push(Operator::Lparen);
                }
                Term::Rparen => loop {
                    match operators.last().unwrap() {
                        Operator::Lparen => {
                            operators.pop().unwrap();
                            break;
                        }
                        Operator::Sentinel => {
                            return Err(Error::UnmatchedParenthError);
                        }
                        _ => Self::push_expr(&mut operands, &mut operators)?,
                    };
                },
                Term::Operator(oper) => {
                    if Self::op_precedence(&oper) == UNARY {
                        operands.push(Operand::Num(0));
                    }
                    while Self::op_precedence(&oper)
                        <= Self::op_precedence(&operators.last().unwrap())
                    {
                        Self::push_expr(&mut operands, &mut operators)?;
                    }
                    operators.push(oper);
                }
                Term::FnArgs(args) => {
                    operands.push(Operand::FnArgs(args));
                    operators.push(Operator::FnBody);
                }
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
                self.last_was_operand = true;
                let token =
                    self.take_input_until(|nc| !nc.is_alphanumeric() && !nc.is_whitespace());
                Some(parse_num(token).map(Term::Num))
            } else if c == '$' {
                self.last_was_operand = true;
                let token = self.take_input_until(|nc| !nc.is_alphanumeric() && nc != '_');
                Some(Ok(Term::Var(token.to_string())))
            } else if c == '|' && !self.last_was_operand {
                self.last_was_operand = true;
                let token = self.take_input_until(|nc| nc == '|');
                let pars = token[1..]
                    .split(',')
                    .map(|par| par.trim().to_string())
                    .collect::<Vec<String>>();
                self.input = &self.input[1..];
                Some(Ok(Term::FnArgs(pars)))
            } else {
                let last_was_operand = self.last_was_operand;
                let token = self.take_input_until(|nc| nc != c || c == '(' || c == ')');
                let oper = match token {
                    "+" => Term::Operator(Operator::Add),
                    "-" if last_was_operand => Term::Operator(Operator::Sub),
                    "-" if !last_was_operand => Term::Operator(Operator::Neg),
                    "*" => Term::Operator(Operator::Mul),
                    "/" => Term::Operator(Operator::Div),
                    "%" => Term::Operator(Operator::Remainder),
                    "**" => Term::Operator(Operator::Pow),
                    "~" => Term::Operator(Operator::BNot),
                    "^" => Term::Operator(Operator::BXor),
                    "|" => Term::Operator(Operator::BOr),
                    "&" => Term::Operator(Operator::BAnd),
                    "<<" => Term::Operator(Operator::LShift),
                    ">>" => Term::Operator(Operator::RShift),
                    "(" => Term::Lparen,
                    ")" => Term::Rparen,
                    "=" => Term::Operator(Operator::Assign),
                    _ => return Some(Err(Error::OperatorParseError(token.to_string()))),
                };
                if let Term::Operator(_) = oper {
                    self.last_was_operand = false;
                }
                Some(Ok(oper))
            }
        } else {
            None
        }
    }
}

#[test]
fn test_lexer() {
    let parser = Parser::new("111 + 222");
    let terms = parser.map(|t| t.unwrap()).collect::<Vec<Term>>();
    assert_eq!(
        format!("{:?}", terms),
        "[Num(111), Operator(Add), Num(222)]"
    );

    let parser = Parser::new("11*22+33");
    let terms = parser.map(|t| t.unwrap()).collect::<Vec<Term>>();
    assert_eq!(
        format!("{:?}", terms),
        "[Num(11), Operator(Mul), Num(22), Operator(Add), Num(33)]"
    );

    let parser = Parser::new("11 ** 22");
    let terms = parser.map(|t| t.unwrap()).collect::<Vec<Term>>();
    assert_eq!(format!("{:?}", terms), "[Num(11), Operator(Pow), Num(22)]");

    let mut parser = Parser::new("11 // 22");
    assert_eq!(parser.next().unwrap().unwrap(), Term::Num(11));
    assert!(parser.next().unwrap().is_err());
}

#[test]
fn test_parser_simple() {
    let oper: Operand = Result::from(Parser::new("1").into()).unwrap();
    assert_eq!(oper, Operand::Num(1));

    let oper: Operand = Result::from(Parser::new("1 * 2").into()).unwrap();
    assert_eq!(format!("{:?}", oper), "Term(Mul, Num(1), Num(2))");

    let oper: Operand = Result::from(Parser::new("0*1+2 * 0x10").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Add, Term(Mul, Num(0), Num(1)), Term(Mul, Num(2), Num(16)))"
    );

    let oper: Operand = Result::from(Parser::new("(2 * 40) - 1").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Sub, Term(Mul, Num(2), Num(40)), Num(1))"
    );

    let oper: Operand = Result::from(Parser::new("1 2 3+4 5  6").into()).unwrap();
    assert_eq!(format!("{:?}", oper), "Term(Add, Num(123), Num(456))");
}

#[test]
fn test_parser_errs() {
    assert!(Result::from(Parser::new("0*").into()).is_err());
    assert!(Result::from(Parser::new("   ").into()).is_err());
    assert!(Result::from(Parser::new("1*2*+3").into()).is_err());
    assert!(Result::from(Parser::new("1*2*").into()).is_err());
}

#[test]
fn test_parser_parenth() {
    let oper: Operand = Result::from(Parser::new("0*(1+2)").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Mul, Num(0), Term(Add, Num(1), Num(2)))"
    );

    let oper: Operand = Result::from(Parser::new("0*((((1+2))))").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Mul, Num(0), Term(Add, Num(1), Num(2)))"
    );

    let oper: Operand = Result::from(Parser::new("0*(1*(2*(3*(4+5))))").into()).unwrap();
    assert_eq!(format!("{:?}", oper), "Term(Mul, Num(0), Term(Mul, Num(1), Term(Mul, Num(2), Term(Mul, Num(3), Term(Add, Num(4), Num(5))))))");

    let oper: Operand = Result::from(Parser::new("0*((1+2)*(3+4))*5").into()).unwrap();
    assert_eq!(format!("{:?}", oper), "Term(Mul, Term(Mul, Num(0), Term(Mul, Term(Add, Num(1), Num(2)), Term(Add, Num(3), Num(4)))), Num(5))");

    assert!(Result::from(Parser::new("0*(1+2))").into()).is_err());
    assert!(Result::from(Parser::new("0*((1+2)").into()).is_err());
}

#[test]
fn test_parser_unary() {
    let oper: Operand = Result::from(Parser::new("-1 * -2").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Mul, Term(Neg, Num(0), Num(1)), Term(Neg, Num(0), Num(2)))"
    );

    let oper: Operand = Result::from(Parser::new("~1 + ((-2))").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Add, Term(BNot, Num(0), Num(1)), Term(Neg, Num(0), Num(2)))"
    );

    let oper: Operand = Result::from(Parser::new("~2 * ~1 - -0x2 * -3").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Sub, \
         Term(Mul, Term(BNot, Num(0), Num(2)), Term(BNot, Num(0), Num(1))), \
         Term(Mul, Term(Neg, Num(0), Num(2)), Term(Neg, Num(0), Num(3))))"
    );
}

#[test]
fn test_parser_assign() {
    let oper: Operand = Result::from(Parser::new("1 = 2 + 3").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Assign, Num(1), Term(Add, Num(2), Num(3)))"
    );

    let oper: Operand = Result::from(Parser::new("a = 1 + 2").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Assign, Num(10), Term(Add, Num(1), Num(2)))"
    );

    let oper: Operand = Result::from(Parser::new("$a = 1 * 2 + 3 / ~4").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Assign, Var(\"$a\"), \
         Term(Add, Term(Mul, Num(1), Num(2)), Term(Div, Num(3), Term(BNot, Num(0), Num(4)))))"
    );

    let oper: Operand = Result::from(Parser::new("$a = $b * $c + $d / ~$e").into()).unwrap();
    assert_eq!(
        format!("{:?}", oper),
        "Term(Assign, Var(\"$a\"), \
         Term(Add, Term(Mul, Var(\"$b\"), Var(\"$c\")), Term(Div, Var(\"$d\"), Term(BNot, Num(0), Var(\"$e\")))))"
    );
}

#[test]
fn test_create_func() {
    let oper: Operand = Result::from(Parser::new("|$x| $x + $y").into()).unwrap();
    assert_eq!(format!("{}", oper), "([\"$x\"] -> ($x + $y))");

    let oper: Operand = Result::from(Parser::new("$z = |$x| $x + $y").into()).unwrap();
    assert_eq!(format!("{}", oper), "($z = ([\"$x\"] -> ($x + $y)))");
}
