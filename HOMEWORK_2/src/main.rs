use std::collections::HashMap;

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

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(10);

    println!("Score: {}", score);

    let matcher = 9;

    match matcher {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6 ..=10 => println!("Six to Ten"),
        _ => println!("Something else"),
    }

    let opt_x  = Some(5);
    let y = 10;

    match opt_x {
        Some(50) => println!("Got 50"),
        Some(b) => println!("Matched, y = {:?}", b),
        _ => println!("Default case, x = {:?}", opt_x),
    }

    println!("At the end x = {:?}, y={:?}", opt_x, y );


    let example_point = Point { x: 0, y: 7 };

    match example_point {
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

}
