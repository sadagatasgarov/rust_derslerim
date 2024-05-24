static mut COUNTER: u32 = 0;

fn increment_counter(){
    unsafe {
        COUNTER+=1;
    }
}
fn main() {
    increment_counter();increment_counter();
    unsafe{
        increment_counter();
        println!("{}", COUNTER);

    }
}
