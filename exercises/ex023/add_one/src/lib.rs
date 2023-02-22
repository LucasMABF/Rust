use rand::Rng; // can't use outside this crate, since its only in tis toml file

pub fn add_one(x: i32) -> i32{
    println!("{}", rand::thread_rng().gen_range(1..=100));
    x + 1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(3, add_one(2));
    }
}
