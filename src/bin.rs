extern crate xc_lib;
extern crate clap;
extern crate rustyline;

use clap::{App, Arg};
use xc_lib::eval::eval_expr;
use xc_lib::show::PresentNum;
use clap::ArgMatches;
use rustyline::{Editor, error::ReadlineError};

fn proc_expr(expr: &str, matches: &ArgMatches) {
    match eval_expr(expr) {
        Ok(res) => {
            let possible_outputs: [(&str, Box<dyn Fn() -> String>); 3] = [
                ("dec", Box::new(|| res.as_dec(true))),
                ("hex", Box::new(|| res.as_hex(true))),
                ("bin", Box::new(|| res.as_bin(true).0)),
            ];
            let mut selected = possible_outputs
                .iter()
                .filter_map(|(name, output)| matches.index_of(name).map(|idx| (idx, output)))
                .collect::<Vec<(usize, &Box<dyn Fn() -> String>)>>();
            selected.sort_by_key(|(idx, _)| *idx);
            if !selected.is_empty() {
                for (_, f) in selected {
                    println!("{}", f());
                }
            } else {
                println!("{}", res.show_all());
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    };
}

fn main() {
    let matches = App::new("xc")
        .version("0.1.0")
        .about("Pocket-sized calculator")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .help("Read expressions from input instead of an argument"),
        )
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
            Arg::with_name("EXPR")
                .help("Expression to calculate")
                .index(1),
        )
        .get_matches();
    if matches.is_present("interactive") {
        let mut editor = Editor::<()>::new();
        loop {
            match editor.readline(">> ") {
                Ok(buf) => {
                    if !buf.trim().is_empty() {
                        proc_expr(&buf, &matches);
                        println!();
                    }
                    editor.add_history_entry(buf);
                },
                Err(ReadlineError::Interrupted) => {},
                _ => break,
            }
        }
    } else if let Some(expr) = matches.value_of("EXPR") {
        proc_expr(expr, &matches);
    } else {
        eprintln!("Please provide either an expression or the --interactive flag");
    }
}
