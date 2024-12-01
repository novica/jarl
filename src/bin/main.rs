use r::message::*;
use r::object::Expr;
use r::parser::*;
use r::session::SessionParserConfig;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let parser = SessionParserConfig::default();
    let r_files = vec!["foo2.R"];
    // let r_files = vec!["foo.R", "foo2.R", "foo3.R"];
    let messages: Vec<Message> = r_files
        .par_iter()
        .map(|file| {
            let file = File::open(Path::new(file)).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ast = parser.parse_input(&contents);
            // let line_col = parser.parse_line_col(&contents);
            // let my_ast = ast.clone().unwrap();
            // println!("{:?}", my_ast);
            // println!("{:?}", line_col);
            check_ast(ast.unwrap())
        })
        .flatten()
        .collect();

    for message in messages {
        println!("{}", message);
    }
    let duration = start.elapsed();
    println!("Checked files in: {:?}", duration);
}

fn check_ast(ast: Expr) -> Vec<Message> {
    let mut messages: Vec<Message> = vec![];
    println!("{}", ast);
    match ast {
        Expr::Call(fun, args) => {
            if *fun.clone() == Expr::Symbol("any".to_string()) {
                if args.len() == 1 {
                    let arg = args.get(0).unwrap();
                    let _mybox = Box::new(Expr::Symbol("is.na".to_string()));

                    match arg {
                        Expr::Call(_mybox, _) => {
                            messages.push(Message::AnyNA {
                                filename: "foobar".into(),
                                location: Location { row: 1, column: 2 },
                            });
                        }
                        _ => unreachable!(),
                    }
                }
                check_ast(*fun.clone());
            }
            let _ = args.into_iter().map(|(_nm, val)| {
                println!("{val}");
                check_ast(val)
            });
        }
        _ => println!("not an expr"),
    }
    messages
}
