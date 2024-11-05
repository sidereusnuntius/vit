use std::{error::Error, io::Read};

use ast::Statement;

pub mod parser;
mod ast;

pub fn run<'a>(config: Config, source: &'a String) -> Result<(), Box<dyn Error + 'a>>{
    let ast = parser::Parser::new().parse(&source)?;
    ast.iter().map(|statement| println!("{:?}", *statement)).collect::<()>();
    
    Ok(())
}

pub struct Config {
    pub file_name: String,
    debug_mode: bool,
    interpret: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_name = if let Some(file) = args.next() {
            file
        } else {
            return Err("No file name given.");
        };

        Ok(Config {
            file_name,
            debug_mode: false,
            interpret: false,
        })
    }
}