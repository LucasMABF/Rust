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
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
    if args.len() < 3{
        return Err("Not enough arguments passed!");
    }
    let query = args[1].clone();
    let file = args[2].clone();
    let mut ignore_case = env::var("IGNORE_CASE").is_ok();
    if args.len() == 4{
        if args[3] == "case_sensitive"{
            ignore_case = false;
        }else if args[3] == "case_insensitive"{
            ignore_case = true;
        }
    }
    Ok(Config{query, file, ignore_case})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut query_lines = vec![];
    for line in contents.lines(){
        if line.contains(query) {
            query_lines.push(line);
        }
    }
    query_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut query_lines = vec![];
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            query_lines.push(line);
        }
    }
    query_lines
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
