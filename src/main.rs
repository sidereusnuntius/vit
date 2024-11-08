use std::{env, process};

fn main() {
    let args = env::args();
    let config = match vit::Config::build(args) {
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        },
        Ok(config) => config,
    };
    
    if let Err(e) = vit::run(config) {
        eprintln!("{:?}", e);
    };
}
