#[link(name = "multiply", kind="static")]
extern "C" {
    fn multiply(a: i32, b: i32) -> i32;
}

fn main() {
    println!("[Rust] Hello from Rust! 🦀");

    unsafe {
        println!("[Rust] Calling function in C..");

        let result = multiply(5i32, 5i32);

        println!("[Rust] Result: {}", result);
    }
}