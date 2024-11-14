use std::{error::Error, fs};

mod ast;
pub mod parser;
pub mod vit;

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error + 'a>> {
    let source_code = fs::read_to_string(&config.file_name)?;
    let program = parser::Parser::new().parse(&source_code)?;
    let result = vit::build(program)?;

    fs::write(config.target_name, result)?;

    Ok(())
}

pub struct Config {
    pub file_name: String,
    pub target_name: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_name = if let Some(file) = args.next() {
            file
        } else {
            return Err("No input file name given.");
        };

        let target_name = args.next().unwrap_or_else(|| file_name.replace(".vit", ""));

        Ok(Config { file_name, target_name })
    }
}
