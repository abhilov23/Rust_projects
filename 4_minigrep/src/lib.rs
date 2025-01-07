use std::error::Error;
use std::fs;

//reading the file in the run function
pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?; //if we face any error, then it will return that

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}




pub struct Config{
    pub query: String,
    pub filename: String,
}

 impl Config{
    //for parsing the files
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { //checking if the length of arguments is less than 3 then panic
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
           results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive
        Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
