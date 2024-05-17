fn main() {
    let greate_than = |x: &i32| *x > 10;
    //let less_than = |x:&i32| *x <20;
    let result = are_both_true(greate_than, less_than, &15);
    println!("{result}")
}


fn less_than(item:&i32) -> bool
{
    *item>20
}

fn are_both_true<T, U, V>(f1: T, f2:U, item: &V) -> bool 
where 
T:Fn(&V)->bool,
U:Fn(&V)->bool
{
    let a = f1(item) && f2(item);
    a
}
