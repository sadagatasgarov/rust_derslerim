fn main() {
    let z = my_function(5);
    println!("my_function returned {}", z)
}


fn my_function(x: i32) -> i32 {
    println!("my)function called with: {}", x);
    let y = 10+x;
    y

}