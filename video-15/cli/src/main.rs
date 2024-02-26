
use std::process;   
use cli::Config;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
   if let Err(e) = cli::run(config) {
       println!("Application error: {}", e);
       process::exit(1);
   }
}