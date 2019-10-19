use crate::error::Result;
use crate::parser::{Operand, Operator, Parser};

pub fn eval_expr(expr: &str) -> Result<i128> {
    let oper: Result<Operand> = Parser::new(expr).into();
    Ok(eval(oper?))
}

fn eval(term: Operand) -> i128 {
    match term {
        Operand::Num(i) => i,
        Operand::Term(op, l, r) => {
            let l = eval(*l);
            let r = eval(*r);
            match op {
                Operator::Add => l + r,
                Operator::Mul => l * r,
                Operator::Sub => l - r,
                Operator::Div => l / r,
                Operator::Remainder => l % r,
                Operator::Neg => -r,
                Operator::BNot => !r,
                Operator::BXor => l ^ r,
                Operator::BOr => l | r,
                Operator::BAnd => l & r,
                Operator::LShift => l << r,
                Operator::RShift => l >> r,
                _ => panic!("Found {:?} in eval", op),
            }
        }
    }
}

#[test]
fn test_eval() {
    assert_eq!(eval_expr("01110b * 0x10 + 1").unwrap(), 225);
    assert_eq!(eval_expr("(0xb*1) + (0*0b1111011111)+1").unwrap(), 12);
    assert_eq!(eval_expr("10 - 5").unwrap(), 5);
    assert_eq!(eval_expr("10 / 3").unwrap(), 3);
    assert_eq!(eval_expr("10 % 3").unwrap(), 1);
    assert_eq!(eval_expr("0b111 ^ 0b101").unwrap(), 0b010);
    assert_eq!(eval_expr("0b010 | 0b100").unwrap(), 0b110);
    assert_eq!(eval_expr("0b011 & 0b101").unwrap(), 0b001);
    assert_eq!(eval_expr("1 << 8").unwrap(), 1 << 8);
    assert_eq!(eval_expr("0xFFFFFFFF >> 8").unwrap(), 0xffffff);
    assert_eq!(eval_expr("(2 * 40) - 1").unwrap(), 79);
}

#[test]
fn test_eval_unary() {
    assert_eq!(eval_expr("-1 * -2").unwrap(), 2);
    assert_eq!(eval_expr("~1 + -10").unwrap(), !1i128 - 10);
    assert_eq!(
        eval_expr("~5 * ~8 - -0x10 * -12").unwrap(),
        !5i128 * !8i128 - 0x10 * 12
    );
}
