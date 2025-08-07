use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug)]
struct ExecutionData {
    symbol: String,
    price: f64,
    quantity: f64,
    side: String,     // "buy" or "sell"
    timestamp: u64,
}

#[tokio::main]
async fn main() {
	let str_request = "ws://127.0.0.1:9001"; //"wss://echo.websocket.org";

    println!("Connecting to {}", str_request);

    let (ws_stream, _) = connect_async(str_request).await.expect("Failed to connect");
    println!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    /*
    // Send a message
    let send_msg = Message::Text("Hello from Rust!".into());
    write.send(send_msg).await.expect("Failed to send");

    // Wait for a response
    if let Some(Ok(msg)) = read.next().await {
        println!("Received: {}", msg);
    } else {
        println!("No response or error.");
    }*/
	
	let trades = vec![
        ExecutionData {
            symbol: "BTCUSD".to_string(),
            price: 45123.45,
            quantity: 0.5,
            side: "buy".to_string(),
            timestamp: 1691493410,
        },
        ExecutionData {
            symbol: "ETHUSD".to_string(),
            price: 3050.12,
            quantity: 1.2,
            side: "sell".to_string(),
            timestamp: 1691493420,
        },
    ];

    for trade in trades {
        let msg = serde_json::to_string(&trade).unwrap();
        println!("Sending: {}", msg);
        write.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await.unwrap();
        sleep(Duration::from_secs(1)).await;
		
		if let Some(Ok(msg)) = read.next().await {
			println!("Received: {}", msg);
		} else {
			println!("No response or error.");
		}
    }

    println!("Done sending trades.");
}