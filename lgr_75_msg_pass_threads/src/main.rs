use std::{sync::mpsc, thread};


fn main() {
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "hello".to_owned(),
        "thrust me!".to_owned(),
    ];

    for i in sentences{
        let tx_clone = tx.clone();
        thread::spawn(move ||{
        let reversed: String = i.chars().rev().collect();
        tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx); // bu  olmasa proqram dayanmir islemeye davam edir ve guce dusur
    for sentence in rx{ 
        println!("{sentence}")
    }

}
