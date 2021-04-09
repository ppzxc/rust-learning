pub fn example_generic1() {
    // if one
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers.get(0).unwrap();

    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // if two
    let numbers = vec![34, 50, 25, 100, 65, 106, 150];

    let mut largest = numbers.get(0).unwrap();

    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    *largest
}

pub fn example_generic2() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
}

// fn largest2<T>(list: &[T]) -> T {
//     let mut largest = list.get(0).unwrap();
//
//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// pub fn example_generic3() {
//     let numbers = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&numbers);
//     println!("The largest number is {}", result);
//
//     let chars = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&chars);
//     println!("The largest char is {}", result);
// }

pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn example_trait1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}