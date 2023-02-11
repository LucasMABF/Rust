// tests
#[cfg(test)] // annotation
mod tests {
    use super::*;

    #[ignore]
    #[test] // atribute marks function as test
    fn it_works() -> Result<(), String>{ // can also return result but with no should_panic
        assert_eq!(2, 2);
        if 2 + 2 == 6{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
        
    }

    #[ignore]
    #[test]
    fn another_test(){
        panic!("make this test fail!!");
    }

    #[test]
    fn yet_another_test(){
        let result = 10 + 100;
        assert_eq!(result, 110); // panics if not equal

        assert_ne!(result, 111); // panics if equal
    }

    #[test]
    fn rectange_test(){
        let larger = Rectangle{width: 100, height: 300};
        let smaller = Rectangle{width: 40, height: 109};
        let alternate = Rectangle{width: 88, height: 400};

        assert!(larger.can_hold(&smaller)); // sees if value is true, panics if not

        assert!(!smaller.can_hold(&larger));

        assert!(!larger.can_hold(&alternate));
    }

    #[ignore]// not to run with cargo test, only when specified
    #[test]
    fn has_name(){
        let name = "Lucas";
        let result = hello(name);
        assert!(result.contains("Lucas"), "msg should contain name: {}, result. Instead value received was {}", name, result); // can have faillure message
    }

    #[should_panic(expected = "Impossible")] // for tests that should panic, if it panics it passes
    // can also see what was the panic message with expected, that checks if this substring is in the panic
    #[test]
    fn triangle_test(){
        Triangle::new(1000, 10, 10);
    }

    #[test]
    fn three_test(){
        assert!(three_plus_three().is_err());
    }

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Triangle{
    side1: i32,
    side2: i32,
    side3: i32,
}


impl Triangle{
    pub fn new(side1: i32, side2: i32, side3: i32) -> Triangle{
        if side1 < side2 + side3 || side2 < side1 + side3 || side3 < side1 + side2{
            panic!("Impossible Triangle");
        }

        Triangle{
            side1,
            side2,
            side3,
        }
    }
}

pub fn hello(name: &str) -> String{
    format!("Hello, name")
}

pub fn three_plus_three() -> Result<i32, String>{
    if 3 + 4 == 6{
        Ok(6)
    } else{
        Err(String::from("3 + 3 is not equal to 6!!"))
    }
}
