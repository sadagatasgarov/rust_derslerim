fn main() {
    let mut m = MyStruct{};
    let item = m.next();
    println!("{:?}", item);
}

trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        None
    }
}


trait IntoIteratot {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
