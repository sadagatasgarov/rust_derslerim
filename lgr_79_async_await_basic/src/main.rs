#[tokio::main]
async fn main() {
  // my_function().await;
   let f = my_function();

   println!("buda esas mainden gelir");
   f.await;
   println!("buda esas mainden gelir");
}

// trait Future {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// fn my_function() -> impl Future<Output = ()> {
//     println!("men async funksiyasiyam amma impl");
// }

enum FutureStateMachine {
    State1,
    State2,
    State3,
}

async fn my_function(){
    println!("men async funksiyasiyam");
    let s1 = read_from_database().await;
    println!("first {}", s1);

    let s2 = read_from_database().await;
    println!("second {}", s2);
}


async fn read_from_database() -> String {
    "DB result".to_string()
}



// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // In a loop, read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     // socket closed
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         eprintln!("failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };

//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }