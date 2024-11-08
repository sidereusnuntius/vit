use std::{collections::HashMap, env, fs, hash::Hash, process};

use vit::{ast, run, Config};

use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

lalrpop_mod!(pub vit_grammar);

fn main() {
    // let result = vit_grammar::ProgramParser::new().parse("input")
    // println!();


    // match result.remove(0) {
    //     ast::Statement::Assignment(id, expr) => println!("{:?}", parse_expression(*expr)),
    //     _ => (),
    // }

    // let args = env::args();
    // let config = match Config::build(args) {
    //     Err(message) => {
    //         eprintln!("{message}");
    //         process::exit(1);
    //     },
    //     Ok(config) => config,
    // };

    // let source = match fs::read_to_string(&config.file_name) {
    //     Ok(source) => source,
    //     Err(message) => {
    //         eprintln!("{message}");
    //         process::exit(1);
    //     }
    // };
    
    // if let Err(e) = run(config, &source) {
    //     eprintln!("{:?}", e);
    // };
}
