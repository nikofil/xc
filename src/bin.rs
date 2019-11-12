extern crate clap;
extern crate rustyline;
extern crate xc_lib;

use clap::{App, Arg};
use rustyline::{error::ReadlineError, Editor};
use xc_lib::eval::{eval_expr, CompResult, Context};
use xc_lib::show::PresentNum;

type OutputFn<'a> = Box<dyn Fn(i128) -> String + 'a>;

fn proc_expr(expr: &str, ctx: &mut Context, out_fns: &[&OutputFn]) {
    match eval_expr(expr, ctx) {
        Ok(Some(CompResult::Num(res))) => {
            if !out_fns.is_empty() {
                for out_fn in out_fns {
                    println!("{}", out_fn(res));
                }
            } else {
                println!("{}", res.show_all());
            }
        }
        Ok(Some(comp_res)) => {
            println!("{}", comp_res);
        }
        Ok(None) => {}
        Err(err) => eprintln!("Error: {}", err),
    };
}

fn main() {
    let matches = App::new("xc")
        .version("0.1.0")
        .about("Pocket-sized calculator")
        .arg(
            Arg::with_name("dec")
                .short("d")
                .help("Only print decimal output"),
        )
        .arg(
            Arg::with_name("hex")
                .short("h")
                .help("Only print hex output"),
        )
        .arg(
            Arg::with_name("bin")
                .short("b")
                .help("Only print binary output"),
        )
        .arg(
            Arg::with_name("expr")
                .multiple(true)
                .help("Expression to calculate, if one isn't given xc opens in interactive mode")
                .index(1),
        )
        .get_matches();

    let possible_outputs: [(&str, OutputFn); 3] = [
        ("dec", Box::new(|res| res.as_dec(true))),
        ("hex", Box::new(|res| res.as_hex(true))),
        ("bin", Box::new(|res| res.as_bin(true).0)),
    ];

    let mut selected = possible_outputs
        .iter()
        .filter_map(|(name, output)| matches.index_of(name).map(|idx| (idx, output)))
        .collect::<Vec<(usize, &OutputFn)>>();

    selected.sort_by_key(|(idx, _)| *idx);

    let selected_ord = selected
        .into_iter()
        .map(|(_, out_fn)| out_fn)
        .collect::<Vec<&OutputFn>>();

    if let Some(exprs) = matches.values_of("expr") {
        let exprs: &str = &exprs.collect::<Vec<&str>>().join(" ");
        let mut ctx = Context::new();
        for expr in exprs.split(';') {
            if !expr.trim().is_empty() {
                if selected_ord.is_empty() {
                    println!("> {}", expr.trim());
                }
                proc_expr(expr, &mut ctx, &selected_ord);
            }
        }
    } else {
        let mut editor = Editor::<()>::new();
        let mut ctx = Context::new();
        loop {
            match editor.readline(">> ") {
                Ok(buf) => {
                    if !buf.trim().is_empty() {
                        proc_expr(&buf, &mut ctx, &selected_ord);
                        println!();
                    }
                    editor.add_history_entry(buf);
                }
                Err(ReadlineError::Interrupted) => {}
                _ => break,
            }
        }
    }
}
