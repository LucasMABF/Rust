//! CLI programm to search for a string pattern in a file
//! and display it in the terminal all the lines in which that pattern is found.
use clap::Parser;
use std::fs::File;
use std::time::Duration;
use anyhow::{Context, Result, ensure};
use std::io::BufReader;
use std::thread;

/// Command line arguments type, to organize the arguments passed by the user.
#[derive(Parser, Debug)]
struct Args{
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    ctrlc::set_handler(move || {
        println!("received Ctrl+C! Exiting");
        std::process::exit(42);
    }).unwrap();

    thread::sleep(Duration::from_secs(2));
    let args = Args::parse();
    ensure!(args.pattern.len() != 0, "no pattern to search for");

    let file = File::open(&args.path).with_context(|| format!("could not read file {:?}", &args.path))?;
    let mut reader = BufReader::new(file);
    grrs::find_str(&args.pattern, &mut reader, &mut std::io::stdout())?;

    Ok(())
}


#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::io::BufReader;
    #[test]
    fn find_a_match(){
        let mut result = Vec::new();
        grrs::find_str("lorem", &mut BufReader::new(File::open("poems/lorem.txt").unwrap()), &mut result).unwrap();
        assert_eq!(result, b"lorem ipsum\n");
    }
}
