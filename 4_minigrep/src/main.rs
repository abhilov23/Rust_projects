#[allow(unused)]
use std::env;
use std::process;
use minigrep::Config; //specifying the filename so that it can import the libraries


fn main() {
    let args: Vec<String> = env::args().collect(); //taking the arguments in form of vector

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //this will terminate the program when facing problem, will remove the extra noise
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("{:?}", args); //printing all the arguments

    if let Err(e) = minigrep::run(config) { //handling the error while calling run function
        println!("Application error: {}", e);
        process::exit(1);
    }
}
