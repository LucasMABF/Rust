// implementation of post with rust's type system pattern instead of oop state implementatin
// this pattern is not fully object oriented, but will throw compile time errors, if misshandled by the user
// could also be implemented with different structs and impl blocks such as PublishedPost, PendingReviewPost, and so forth
// however I will use a more advanced and less repetitive implementation using PhantomData
// that is a data type only avaiable at compile time.

pub struct Draft;
pub struct PendingReview;
pub struct PendingReview1;
pub struct Published;

pub struct Post<State = Draft>{
    content: String,
    state: std::marker::PhantomData<State>,
}

impl Post{
    pub fn new() -> Post<Draft>{
        Post{
            content: String::new(),
            state: std::marker::PhantomData::<Draft>,
        }
    }
}


impl Post<Draft>{
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn request_review(self) -> Post<PendingReview>{
        Post{
            content: self.content,
            state: std::marker::PhantomData::<PendingReview>,
        }
    }
}

impl Post<PendingReview>{
    pub fn approve(self) -> Post<PendingReview1>{
        return Post{
            content: self.content,
            state: std::marker::PhantomData::<PendingReview1>,
        }
    }

    pub fn reject(self) -> Post<Draft>{
        Post{
            content: self.content,
            state: std::marker::PhantomData::<Draft>,
        }
    }
}

impl Post<PendingReview1>{
    pub fn approve(self) -> Post<Published>{
        Post{
            content: self.content,
            state: std::marker::PhantomData::<Published>,
        }
    }
}

impl Post<Published>{
    pub fn content(&self) -> &str{ // lifetimes are automatically assumed, due to ellision
        &self.content
    }
}

