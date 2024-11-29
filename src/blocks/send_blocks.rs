use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
	let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
	let bloco = "New block mined!".to_string();

	stream.write_all(bloco.as_bytes()).await.unwrap();

	let mut buffer = [0; 1024];
	let n = stream.read(&mut buffer).await.unwrap();
	println!("Response: {}", String::from_utf8_lossy(&buffer[..n]));
}
