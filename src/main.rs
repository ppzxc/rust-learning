////////////////////////////////////////
/// 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
// extern crate aggregator;

// use aggregator::Summarizable;
// use aggregator::Tweet;

// struct WeatherForecast {
//     high_temp: f64,
//     low_temp: f64,
//     chance_of_precipitation: f64,
// }

// impl Summarizable for WeatherForecast {
//     fn summary(&self) -> String {
//         format!("The high will be {}, and the low will be {}. The chance of
//         precipitation is {}%.", self.high_temp, self.low_temp,
//         self.chance_of_precipitation)
//     }
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summary());
// }

/////////////////////////////////////////
// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn main() {
//     let wont_work = Point { x: 5, y: 4 };
//     println!("p.x = {}", wont_work.x());

//     let p1 = Point2 { x: 5, y: 10.4 };
//     let p2 = Point2 { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point2<T, U> {
//     fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
//         Point2 {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("Hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
    
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("heloo.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }