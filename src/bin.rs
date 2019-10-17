extern crate xc_lib;
extern crate clap;

use xc_lib::eval::eval_expr;
use clap::{Arg, App};

fn main() {
    let matches = App::new("xc")
        .version("0.1.0")
        .about("Pocket-sized calculator")
        .arg(Arg::with_name("EXPR")
            .help("Expression to calculate")
            .required(true)
            .index(1))
        .get_matches();
    let expr = matches.value_of("EXPR").unwrap();
    match eval_expr(expr) {
        Ok(res) => println!("{}", res),
        Err(err) => eprintln!("Error: {}", err),
    };
}
