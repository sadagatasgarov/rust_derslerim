
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello World! bu da makridan makri
        ");
    };
}

#[macro_export]
macro_rules! map {
    ($key:ty, $val:ty) => {
        {
            let map5: HashMap<$key, $val> = HashMap::new();
            map5
        }
    };

    ($($key:expr => $val:expr), *) => {
        {
            let mut map5 = HashMap::new();
            $(map5.insert($key, $val);)*
            map5
        }
    }
}