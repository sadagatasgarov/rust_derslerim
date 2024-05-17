

struct Person {
    first_name: String,
    last_name: String,
    occupation: String
}

#[derive(Debug)]
struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty(){
            return None;
        }
        Some(self.values.remove(0))
    }
}


impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;
    fn into_iter(self) -> Self::IntoIter {
        PersonIterator{
            values: vec![
                self.first_name,
                self.last_name,
                self.occupation
            ]
        }
    }
}

fn main() {

    let p = Person{
        first_name: "Sadagat".to_owned(),
        last_name:"Asgarov".to_owned(),
        occupation:"S engineer".to_owned(),

    };

    let mut a = p.into_iter();
    // let b = p.into_iter().next();
    // let c = p.into_iter().next().unwrap();

    println!("{:?}",a);

    println!("{:?}",a.next());

}
