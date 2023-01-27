//pub waitlist: Vec::<&str> = Vec::new();
pub fn add_to_waitlist(name: &str){
    //waitlist.push(name);
    println!("added {name} to waitlist.");
    //println!("{:?}", waitlist);
}

pub fn seat_at_table(){
    //seat(table_avaiable);
    super::serving::activate_serving(); // accessing front_of_house through super and a relative path.
}
