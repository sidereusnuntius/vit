use std::{error::Error, fs};

pub mod parser;
mod ast;
pub mod vit;

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error + 'a>>{
    let source_code = fs::read_to_string(&config.file_name)?;
    let program = parser::Parser::new().parse(&source_code)?;
    let result = vit::build(program)?;

    fs::write(config.file_name.replace(".vit", ""), result)?;

    Ok(())
}

pub struct Config {
    pub file_name: String,
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
        })
    }
}