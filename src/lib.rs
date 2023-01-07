use std::fs;
use std::error::Error;
use std::env;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("no enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = if args.len() == 4 { args[3].clone()} else {"".to_string()};
    
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if case_insensitive.to_lowercase().eq("case_insensitive"){
            case_sensitive = false;
        }

        Ok(Config { query, filename, case_sensitive})
    }
}
pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for lines in contents.lines(){
        if lines.contains(query){
            results.push(lines);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for lines in contents.lines(){
        if lines.to_lowercase().contains(&query){
            results.push(lines);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query,contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}