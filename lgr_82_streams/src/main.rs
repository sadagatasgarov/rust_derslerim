use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = 
    tokio_stream::iter(["A", "b", "c"]).map(|s|{
        s.to_ascii_lowercase()
    });

    while let Some(s) = stream.next().await {
        println!("{s}")
    }
}