use std::collections::HashMap; // HashMaps are not automatically in scope on root
use ex015;

fn main() {
    // HashMaps
    // Hashmaps is a collection that connects a key and a value, just like Dictionaries in python
    // they are of type HashMap<K, V).
    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue Team"), 10);
    scores.insert(String::from("Yellow Team"), 50);

    println!("{:?}", scores);
    // get value from key
    println!("Blue Team score: {}", scores.get("Blue Team").copied().unwrap_or(0));

    
    let new_team = String::from("Red Team");
    let new_team_score = 100;
    scores.insert(new_team, new_team_score); // takes ownership of String and copies i32
    // println!("{new_team}"); can't value moved
    println!("{new_team_score}"); // we could creae a hashmap with references too, with type &String or &i32.

    // iterate
    for (key, value) in &scores{ // without & scores moved and invalid after
        println!("{key}: {value}");
    }

    // updating hashmap

    scores.insert(String::from("Red Team"), 80); // overwrites old value;

    scores.entry(String::from("Blue Team")).or_insert(20); // only inserts if there wasn't a value before
    scores.entry(String::from("Green Team")).or_insert(120);

    let blue_team_score_change = scores.entry(String::from("Blue Team")).or_insert(20); // returns a mutable reference to the value

    *blue_team_score_change += 200;

    for (key, value) in &scores{ // without & scores moved and invalid after
        println!("{key}: {value}");
    }

    let text = "hello how are you hello you how how how hi";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    for (key, value) in &map{
        println!("{key}: {value}");
    }

    let v = vec![1, 2, 3, 4, 1, 1, 9, 9, 9, 9];
    let med = ex015::med(&v);
    println!("{:?}", med);
    println!("{:?}", v);

    let txt = String::from("Hello how are you today first apple yesterday rust");
    let pig_latin = ex015::pig_latin(&txt);
    println!("{}", pig_latin);
    println!("{}", txt);

    ex015::cli_tool();

}
