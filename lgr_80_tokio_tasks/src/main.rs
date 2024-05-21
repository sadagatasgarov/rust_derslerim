use tokio::time::sleep;
use tokio::time::Duration;


#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}


async fn my_function(i: i32) {
    println!("[{i}] I am async func {i}");

    let s1 = read_from_database().await;
    println!("[{i}] birinci {s1}");

    let s2 = read_from_database().await;
    println!("[{i}] ikinci {s2}");
    
}

async fn read_from_database() -> String{
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}