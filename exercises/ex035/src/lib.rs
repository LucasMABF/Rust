use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};

pub fn find_str(pattern: &str, content: &mut BufReader<File>, mut writer: impl std::io::Write) -> Result<()>{
    for line in content.lines(){
        if let Ok(line) = line{
            if line.contains(pattern){
            writeln!(writer, "{}", line).with_context(|| format!("could not write to writer buffer"))?;
            }
        }
    }
    Ok(())
}
