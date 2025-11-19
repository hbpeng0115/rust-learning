use aggregator::{SocialPost, NewsArticle, Summary};
use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let _integer = Point {x: 5, y: 10};
    let _float = Point {x: 1.0, y: 4.0};
    let _integer_and_float = PointI {x: 4, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p1 = PointI { x: 5, y: 10.4 };
    let p2 = PointI { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display, {
    println!("Announcement: {ann}");
    if x.len() > y.len() { x } else { y }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct PointI<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> PointI<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointI<X2, Y2>) -> PointI<X1, Y2> {
        PointI {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}