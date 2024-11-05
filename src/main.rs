use std::{env, process, fs};

use vit::{parser, run, Config};

fn main() {
    let args = env::args();
    let config = match Config::build(args) {
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        },
        Ok(config) => config,
    };

    let source = match fs::read_to_string(&config.file_name) {
        Ok(source) => source,
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };
    
    if let Err(e) = run(config, &source) {
        eprintln!("{:?}", e);
    };
}
