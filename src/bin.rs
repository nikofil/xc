extern crate xc_lib;
use xc_lib::eval::eval_expr;

fn main() {
    match eval_expr("1+2") {
        Ok(res) => println!("Hello, world! {}", res),
        Err(err) => eprintln!("Error: {}", err),
    };
}
