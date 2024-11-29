use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("Running on 127.0.0.1:8080");

	loop {
		let (mut socket, _) = listener.accept().await.unwrap();
		let mut buffer = [0; 1024];
		let n = socket.read(&mut buffer).await.unwrap();
		println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

		socket.write_all(b"Block received!\n").await.unwrap();
	}
}
