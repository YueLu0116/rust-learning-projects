pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

// **notes:
// The book said:
// To consume the old state, the request_review method needs to take ownership of the state value. 
// This is where the Option in the state field of Post comes in: 
// we call the take method to take the Some value out of the state field and leave a None in its place, 
// because **Rust doesn’t let us have unpopulated fields in structs.**  
// This lets us move the state value out of Post rather than borrowing it. 
// Then we’ll set the post’s state value to the result of this operation.
// We need to set state to None temporarily 
// rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value. 
// This ensures Post can’t use the old state value after we’ve transformed it into a new state.
// ** not fully understand yet.

impl Post{
    pub fn new()->Post{
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self)->&str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve());
        }
    }
}

// stata patterns
trait State{
    fn request_review(self : Box<Self>)->Box<dyn State>;
    // Box<Self> means: the method is only valid when called on a Box holding the type. 
    // invalidating the old state so the state value of the Post
    // and transform into a new state.
    fn approve(self: Box<Self>)->Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post)->&'a str{
        ""
    }
}

struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>)->Box<dyn State>{
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>)->Box<dyn State>{
        self
    }
}

struct PendingReview{}

impl State for PendingReview{
    fn request_review(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn approve(self:Box<Self>)->Box<dyn State>{
        Box::new(Published{})
    }
}

struct Published{}

impl State for Published{
    fn request_review(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>)->Box<dyn State>{
        self
    }
    fn content<'a>(&self, post: &'a Post)->&'a str{
        &post.content
    }
}