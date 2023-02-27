// object oriendet design pattern, with state structs

pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post{
    pub fn new() -> Post{
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.reject())
        }
    }
}

trait State{
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn add_text<'a>(&self, text: &'a str) -> &'a str{
        ""
    }
}

struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{approve_count: 0})
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
    
    fn add_text<'a>(&self, text: &'a str) -> &'a str{
        text
    }
}

struct PendingReview {
    approve_count: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State>{
        self.approve_count += 1;
        if self.approve_count == 2{
            return Box::new(Published {});
        }
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
}
