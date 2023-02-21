use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let content = fs::read_to_string(config.file)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else{
        search(&config.query, &content)
    };

    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub struct Config{
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
    args.next();

    let query = match args.next(){
        Some(arg) => arg,
        None => return Err("Not enough arguments passed: Didn't get a query string."),
    };

    let file = match args.next(){
        Some(arg) => arg,
        None => return Err("Not enough arguments passed: Didn't get a file path"),
    };

    let ignore_case: bool = match args.next(){
        Some(arg) => {
            if arg == "case_sensitive"{
                false
            } else if arg == "case_insensitive"{
                true
            }else{
                env::var("IGNORE_CASE").is_ok()
            }
        },
        None => env::var("IGNORE_CASE").is_ok(),
    };
    Ok(Config{query, file, ignore_case})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";


        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    
}
