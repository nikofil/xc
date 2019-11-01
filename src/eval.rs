use crate::error::Result;
use crate::parser::{Operand, Operator, Parser};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum CompResult {
    Num(i128),
    Func(Vec<String>, Operand),
}

impl Display for CompResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompResult::Num(i) => write!(f, "{}", i),
            CompResult::Func(args, body) => write!(f, "|{}| {}", args.join(", "), body),
        }
    }
}

pub type Context = HashMap<String, CompResult>;

pub fn eval_expr(expr: &str, ctx: &mut Context) -> Result<Option<CompResult>> {
    let oper: Result<Operand> = Parser::new(expr).into();
    Ok(eval(oper?, ctx))
}

fn eval(term: Operand, ctx: &mut Context) -> Option<CompResult> {
    match term {
        Operand::Num(i) => Some(CompResult::Num(i)),
        Operand::Term(Operator::Assign, lhs, term) => {
            if let Operand::Var(var_name) = *lhs {
                let x = eval(*term, ctx)?;
                ctx.insert(var_name, x);
            }
            None
        }
        Operand::Var(var_name) => ctx.get(&var_name).cloned(),
        Operand::Term(Operator::FnBody, args, body) => {
            if let Operand::FnArgs(args) = *args {
                Some(CompResult::Func(args, *body))
            } else {
                panic!("Found FnBody without arguments");
            }
        }
        Operand::Term(Operator::FnCall, method, actual_args) => {
            let method_decl = eval(*method, ctx)?;
            if let CompResult::Func(formal_args, func_body) = method_decl {
                let mut args: Vec<CompResult> = Vec::new();
                let mut cur = Some(*actual_args);
                while let Some(Operand::Term(Operator::List, left, right)) = cur {
                    args.push(eval(*right, ctx).unwrap());
                    cur = Some(*left);
                }
                if let Some(cur) = cur.take() {
                    args.push(eval(cur, ctx).unwrap())
                }
                let mut ctx = Context::new();
                formal_args
                    .iter()
                    .zip(args.iter().rev())
                    .for_each(|(actual, formal)| {
                        ctx.insert(actual.clone(), formal.clone());
                    });
                eval(func_body, &mut ctx)
            } else {
                None
            }
        }
        Operand::Term(op, l, r) => {
            if let (CompResult::Num(l), CompResult::Num(r)) = (eval(*l, ctx)?, eval(*r, ctx)?) {
                Some(CompResult::Num(match op {
                    Operator::Add => l + r,
                    Operator::Mul => l * r,
                    Operator::Sub => l - r,
                    Operator::Div => l / r,
                    Operator::Remainder => l % r,
                    Operator::Pow => l.pow(r as u32),
                    Operator::Neg => -r,
                    Operator::BNot => !r,
                    Operator::BXor => l ^ r,
                    Operator::BOr => l | r,
                    Operator::BAnd => l & r,
                    Operator::LShift => l << r,
                    Operator::RShift => l >> r,
                    _ => panic!("Found {:?} in eval", op),
                }))
            } else {
                None
            }
        }
        other => panic!("Found unexpected operand: {:?}", other),
    }
}

#[test]
fn test_eval() {
    let test_eval = |expr| {
        if let CompResult::Num(i) = eval_expr(expr, &mut HashMap::new()).unwrap().unwrap() {
            i
        } else {
            panic!("Not a number!");
        }
    };
    assert_eq!(test_eval("01110b * 0x10 + 1"), 225);
    assert_eq!(test_eval("(0xb*1) + (0*0b1111011111)+1"), 12);
    assert_eq!(test_eval("10 - 5"), 5);
    assert_eq!(test_eval("10 / 3"), 3);
    assert_eq!(test_eval("10 % 3"), 1);
    assert_eq!(test_eval("0b111 ^ 0b101"), 0b010);
    assert_eq!(test_eval("0b010 | 0b100"), 0b110);
    assert_eq!(test_eval("0b011 & 0b101"), 0b001);
    assert_eq!(test_eval("1 << 8"), 1 << 8);
    assert_eq!(test_eval("0xFFFFFFFF >> 8"), 0xffffff);
    assert_eq!(test_eval("(2 * 40) - 1"), 79);
    assert_eq!(test_eval("3 ** 3 - 1"), 26);
}

#[test]
fn test_eval_unary() {
    let test_eval = |expr| {
        if let CompResult::Num(i) = eval_expr(expr, &mut HashMap::new()).unwrap().unwrap() {
            i
        } else {
            panic!("Not a number!");
        }
    };
    assert_eq!(test_eval("-1 * -2"), 2);
    assert_eq!(test_eval("~1 + -10"), !1i128 - 10);
    assert_eq!(
        test_eval("~5 * ~8 - -0x10 * -12"),
        !5i128 * !8i128 - 0x10 * 12
    );
}

#[test]
fn test_eval_ctx() {
    let mut ctx = HashMap::new();
    assert_eq!(eval_expr("$x = 1", &mut ctx).unwrap(), None);
    assert_eq!(eval_expr("$x", &mut ctx).unwrap(), Some(CompResult::Num(1)));
    assert_eq!(eval_expr("$y = ($x*3) << ($x+1)", &mut ctx).unwrap(), None);
    assert_eq!(eval_expr("$x = -$x * $y", &mut ctx).unwrap(), None);
    assert_eq!(
        eval_expr("$x", &mut ctx).unwrap(),
        Some(CompResult::Num(-12))
    );
}

#[test]
fn test_eval_funcs() {
    let mut ctx = HashMap::new();
    assert_eq!(
        eval_expr("$f = |$x, $y, $z| ($x+$y)*$z", &mut ctx).unwrap(),
        None
    );
    assert_eq!(
        eval_expr("$f", &mut ctx).unwrap().unwrap().to_string(),
        "|$x, $y, $z| (($x + $y) * $z)"
    );

    assert_eq!(
        eval_expr("$f(1, 2, 3)", &mut ctx).unwrap(),
        Some(CompResult::Num(9))
    );

    assert_eq!(
        eval_expr(
            "(|$i, $j| (|$x, $y, $z| $x * $y + $z)($i, 2, $j))(3, 1)",
            &mut ctx
        )
        .unwrap(),
        Some(CompResult::Num(7))
    );
}
