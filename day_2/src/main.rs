use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = day_2::get_file_path(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("File path: {}", input_file);

    if let Err(e) = day_2::run(&input_file) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
