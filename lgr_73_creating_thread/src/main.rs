use std::thread;

fn main() {
    let handle = thread::spawn({||{
        for i in 0..=1000000{
            println!("{i}");
          //  thread::sleep(Duration::from_millis(100));

        }   
 
    }});


    let handle2 = thread::spawn({||{
        for i in 0..=1000000{
            println!("{i}");
           // thread::sleep(Duration::from_millis(50));

        }   
    }});
    for i in 0..=1000000{
        println!("{i}");
        //thread::sleep(Duration::from_millis(50));

    }  
    handle.join().unwrap();
    handle2.join().unwrap();
}
