// RefCell interior mutability smart pointer
// can give many immutable references
// but only one mutable
// however if these rules are broken
// the code will still compile but panic at runtime.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger,{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>{
        LimitTracker{
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let percent_of_max = self.value as f64 / self.max as f64;

        if percent_of_max >= 1.0{
            self.messenger.send("Error: You are over your quota!");
        } else if percent_of_max >= 0.9{
            self.messenger.send("Urgent Warning: You've used up over 90% of your quota");
        } else if percent_of_max >= 0.75{
            self.messenger.send("Warning: You've used up over 75% of your quota");
        } else{
            self.messenger.send("You used less then 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger{
            MockMessenger{
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, message: &str){
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn message_75(){
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);
        tracker.set_value(80);
        assert_eq!(mock.sent_messages.borrow().len(), 1);
    }
}
