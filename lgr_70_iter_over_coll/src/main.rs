use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_string(), 2);
    scores.insert("blue team".to_string(), 8);
    scores.insert("green team".to_string(), 6);

    for (t, s) in scores{
        println!("{} {}", t, s)
    }

}
