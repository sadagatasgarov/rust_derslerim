fn main() {
   
    //1 Dereference a raw pointer
    let mut s = "alfabe".to_owned();
    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;
    let address = 0x012345usize;
    let raw3 = address as *const String;

    
    unsafe {
        //1 Dereference a raw pointer
        (*raw2).push_str("!!!");
        println!("raw2 is: {:?} {:?} {:p}", raw2, raw1, &s);

        
        //2 Call an unsafe function
        my_function();

        //3 Implement an unsafe trait

        let a = String::new();

        //4 Access/Modify a mutable static variable
        //5 Acces fields of a union

    }
}


unsafe fn my_function() {
    println!("calling my function");
}


unsafe trait MyTrait {
    fn some_function(&self);
}

unsafe impl MyTrait for String {
    fn some_function(&self) {
        // ---
    }
}