

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Point<T> {
    x: T,
    y: T,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Point<i32> {
    fn summarize(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

impl Summary for Point<String> {
    fn summarize(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}


fn iterator_demo() {
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = & mut v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
}



fn main() {
    let mut array = [1, 2, 3, 4, 5];
    println!("Array before: {:?}", array);
    array.reverse();
    println!("Array after: {:?}", array);


    let mut s = String::from("Hello, world!");

    let s1 = &mut s;
    s1.push_str(" Laurence");

    println!("{}", s1);


    let a =20;
    let b = &a;

    if (*b ==20) {
        println!("b is equal to 20");
    } else {
        println!("b is not equal to 20");
    }


    let tweet = Tweet {
        username: String::from("Laurence"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("Laurence"),
        content: String::from("Hello, world!"),
    };

    println!("Summary: {}", tweet.summarize()); 
    println!("Summary: {}", news_article.summarize());

    let my_point = Point { x: 5, y: 10 };

    let my_point_str = Point { x: "Hello", y: "World" };


    println!("Summary: {}", my_point.summarize());


    iterator_demo();
}
