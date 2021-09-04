pub trait Summary{

    fn summarize_author(&self)->String;
    fn summarize(&self)->String{
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct TheNewYorkers{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct TikTok{
    pub username: String,
    pub video_type: String,
    pub reply: String,
}

impl Summary for TheNewYorkers{
    fn summarize_author(&self)->String{
        format!("Wow, {} is writing for tnk", self.author)
    }
    fn summarize(&self)->String{
        format!("{}: {}", self.author, self.headline)
    }
}

impl Summary for TikTok{

    fn summarize_author(&self)->String{
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary){    // sugar for notify<T: Summary>(item: &T){}
    println!("Breaking! {}", item.summarize())
}

fn get_summarize()->impl Summary{
    TikTok{
        username: String::from("Long Ciyu"),
        video_type: String::from("Adults only"),
        reply: String::from("sao ji"),
    }
}


fn main(){
    let tnk = TheNewYorkers{
        headline: String::from("Life After White-Collar Crime"),
        location: String::from("The United States"),
        author: String::from("Evan Osnos"),
        content: String::from("Every week, fallen executives come together, seeking sympathy and a second act."),
    };
    println!("I read The New Yorkers today: {}", tnk.summarize());

    let tok = TikTok{
        username: String::from("Long Ciyu"),
        video_type: String::from("Adults only"),
        reply: String::from("sao ji"),
    };
    println!("New tiktok: {}", tok.summarize())
}