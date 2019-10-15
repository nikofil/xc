use crate::parser::{Operand, Operator, Parser};
use crate::error::Result;

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
                _ => panic!("Found {:?} in eval", op),
            }
        },
    }
}

#[test]
fn test_eval() {
    assert_eq!(eval_expr("01110b * 0x10 + 1").unwrap(), 225);
    assert_eq!(eval_expr("(0xb*1) + (0*0b1111011111)+1").unwrap(), 12);
}
