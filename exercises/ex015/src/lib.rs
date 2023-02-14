// exercises from book chapter 08
use std::collections::HashMap;
use std::io::Write;
use std::io;

pub fn med(v: &Vec<i32>) -> (i32, i32) {
    let v: Vec<i32> = v.clone();
    let me: i32 = *v.get(v.len() / 2).unwrap_or(&0);
    let mut mo = HashMap::new();
    for value in &v{
         let c = mo.entry(*value).or_insert(0);
         *c += 1;
    }

    let mut most = 0;
    let mut most_key = 0;
    for (key, value) in &mo{
        if *value > most{
            most = *value;
            most_key = *key;
        }
    }
    (me, most_key)
}

pub fn pig_latin(txt: &str) -> String {
    let mut full_string = String::from("");
    for word in txt.split_whitespace(){
        match &word[0..1]{
            "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => {
                full_string.push_str(&word[..]);
                full_string.push_str("-hay ");
            },
            _ => {  
                    full_string.push_str(&word[1..]);
                    full_string.push_str("-");
                    full_string.push_str(&word[0..1]);
                    full_string.push_str("ay ");
                },

        }
    }
    String::from(full_string)
}

pub fn cli_tool(){
    println!("{:-^90}", "Employee managment system");
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
        
    loop{
        let mut command = String::new();
        print!("$");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        command = command.trim().to_string();
        let command: Vec<&str> =  command.split(" ").collect();

        match command[0].to_lowercase().as_str(){
            "add" => {
                if command.len() < 4{
                    println!("Input a valid command!");
                    continue;
                }
                let mut name = String::new();
                for names in &command{
                        if *names == "to"{
                            break;
                        } else if *names != "add"{
                            name.push_str(names);
                        }
                        name.push(' ');
                    }
                let lst = data.entry(String::from(*command.get(command.len() - 1).expect("Error on adding"))).or_insert(vec!());
                lst.push(name.trim().to_string());
                lst.sort();
            },
            "list" => {
                match command[1].to_lowercase().as_str(){
                    "all" => {
                        for (key, value) in &data {
                            println!("{:-^30}", key);
                            for v in value{
                                println!("{}", v);
                            }
                        }
                    },
                    section => {
                        for v in data.get(section).unwrap(){
                            println!("{}", v);
                        }
                    },
                }
            },
            "quit" => break,
            _ => {
                println!("Input a valid command!");
            },
        }
    }
}
