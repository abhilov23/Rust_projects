#[allow(unused)]
use std::env;
use std::error::Error;
use std::fs;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect(); //taking the arguments in form of vector

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //this will terminate the program when facing problem, will remove the extra noise
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("{:?}", args); //printing all the arguments

    if let Err(e) = run(config) { //handling the error while calling run function
        println!("Application error: {}", e);
        process::exit(1);
    }
}

//reading the file in the run function
fn run(config:Config)-> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?; //if we face any error, then it will return that
    println!("With text: {}", contents);
    Ok(())
}


struct Config{
    query: String,
    filename: String,
}

impl Config{
    //for parsing the files
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { //checking if the length of arguments is less than 3 then panic
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}


