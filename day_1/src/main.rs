use std::env;
use std::process;

use day_1::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Bin: {}, file: {}", cfg.binary, cfg.file);

    if let Err(e) = day_1::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

