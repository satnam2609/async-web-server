use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080".to_string())
        .await
        .unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    println!("processing server");
    let mut buf = vec![0; 1024];
    socket.read(&mut buf).await.unwrap();
    sleep(Duration::from_millis(3000)).await;
    let contents = "finished processing";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-type:text/plain\r\nContent-length:{}\r\n\r\n{}",
        contents.len(),
        contents
    );

    socket.write_all(response.as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    println!("finished processing");
}
