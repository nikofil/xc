use crate::error::Result;
use crate::parser::{Operand, Operator, Parser};
use std::collections::HashMap;

pub fn eval_expr(expr: &str, ctx: &mut HashMap<String, i128>) -> Result<Option<i128>> {
    let oper: Result<Operand> = Parser::new(expr).into();
    Ok(eval(oper?, ctx))
}

fn eval(term: Operand, ctx: &mut HashMap<String, i128>) -> Option<i128> {
    match term {
        Operand::Num(i) => Some(i),
        Operand::Term(Operator::Assign, box Operand::Var(var_name), term) => {
            let x = eval(*term, ctx)?;
            ctx.insert(var_name, x);
            None
        }
        Operand::Var(var_name) => ctx.get(&var_name).map(|num| *num),
        Operand::Term(op, l, r) => {
            let l = eval(*l, ctx)?;
            let r = eval(*r, ctx)?;
            Some(match op {
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
            })
        }
    }
}

#[test]
fn test_eval() {
    let test_eval = |expr| eval_expr(expr, &mut HashMap::new()).unwrap().unwrap();
    assert_eq!(test_eval("01110b * 0x10 + 1"), 225);
    assert_eq!(
        test_eval("(0xb*1) + (0*0b1111011111)+1"),
        12
    );
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
    let test_eval = |expr| eval_expr(expr, &mut HashMap::new()).unwrap().unwrap();
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
    assert_eq!(eval_expr("$x", &mut ctx).unwrap(), Some(1));
    assert_eq!(eval_expr("$y = ($x*3) << ($x+1)", &mut ctx).unwrap(), None);
    assert_eq!(eval_expr("$x = -$x * $y", &mut ctx).unwrap(), None);
    assert_eq!(eval_expr("$x", &mut ctx).unwrap(), Some(-12));
}
