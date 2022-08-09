pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X, Y> {
    x: X,
    y: Y,
}

impl Point<f32,f32> {
    fn distance_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'A', 'z', '@', 'e'];


    println!("The largest char is {}", largest(&char_list));
    println!("The largest number is {}", largest(&number_list));

    let point_map = Point { x: 10.3, y: 4.2 };
    let point_game = Point { x:1 , y:2 };
    let point = Point { x:1 , y:2.2 };

    println!("{}", point_game.x);
    println!("{}", point.y);

    println!("A distancia do ponto pra origem e de {:?}", point_map.distance_origin());

    let tweet = Tweet { 
        username: String::from("Hiago"),
        content: String::from("Bla blla"),
        reply: false,
        retweet: false,
     };

     println!("Sumario do tweet {}", tweet.summarize());


     let string1 = String::from("abcd");

     println!("{}", string1);
     println!("{}", string1.as_str());
}
