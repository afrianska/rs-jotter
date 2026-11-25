use std::{env, process};
use xiaogrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    let query = &args[1];
    let file_path = &args[2];
    */

    // Extracting the argument parser
    // let config = parse_config(&args);

    // Creating a constructor for config
    //let config = Config::new(&args);

    // Returnng a result instead of calling panick.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");

        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!();

    //run(config);
    if let Err(e) = xiaogrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
