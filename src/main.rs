fn main() {
    let p = Post {
        body: "this quick brown fox jumps over the lazy dog".to_string(),
    };
    let summary = p.summarize();
    println!("{}",summary);
}

pub struct Post {
    pub body: String,
}

impl Summary for Post {
    fn summarize( &self) -> String {
    let ret = self.body.get(0..10).unwrap();
    ret.to_string()
    }
}


pub trait Summary {
    fn summarize(&self) -> String;
}