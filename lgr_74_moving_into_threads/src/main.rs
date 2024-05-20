use std::thread;

fn main() {
    let s = "Oyrenirik threadlari".to_string();

    let handle = thread::spawn(move || println!("{}", s));

    handle.join().unwrap();
}
