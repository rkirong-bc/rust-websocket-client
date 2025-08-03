use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
	let str_request = "ws://127.0.0.1:9001"; //"wss://echo.websocket.org";

    println!("Connecting to {}", str_request);

    let (ws_stream, _) = connect_async(str_request).await.expect("Failed to connect");
    println!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    // Send a message
    let send_msg = Message::Text("Hello from Rust!".into());
    write.send(send_msg).await.expect("Failed to send");

    // Wait for a response
    if let Some(Ok(msg)) = read.next().await {
        println!("Received: {}", msg);
    } else {
        println!("No response or error.");
    }
}