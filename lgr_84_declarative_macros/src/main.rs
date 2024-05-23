use std::{collections::HashMap, iter::Scan};

use lgr_84_declarative_macros::*;



fn main() {
    println!("Hello, world!");
    hello!();
    let scores: HashMap<String, i32> = HashMap::new();

    let score = map!(String, i32);

    let mut scores2 = HashMap::new();
    scores2.insert("Red ream".to_owned(), 3);
    scores2.insert("Blue team".to_owned(), 5);
    scores2.insert("Green ream".to_owned(), 2);

    let score2 = map!(
       "dsd".to_string() => 3
    );

    println!("{:?} {:?} {:?}", score, score2, scores2)
}
