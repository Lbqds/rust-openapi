mod types;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{
        connect_async,
        tungstenite::{Message, Utf8Bytes},
    };

    async fn subscribe(params: SubscribeParams) {
        let url = "ws://127.0.0.1:12973/ws";
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        let request = build_subscribe_request(params, 0).unwrap();
        write
            .send(Message::Text(Utf8Bytes::from(request)))
            .await
            .unwrap();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let response: WsResponse = serde_json::from_str(text.as_str()).unwrap();
                    println!("Received response: {:?}", response);
                }
                Ok(other) => {
                    println!("Received non-text message: {:?}", other);
                }
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_subscribe_event() {
        let addresses = vec![
            "tpxjsWJSaUh5i7XzNAsTWMRtD9QvDTV9zmMNeHHS6jQB".to_string(),
            "29YL53teVrvK2o4P2cVej8aCGN7iQS8mE86bgxA2oFWa3".to_string(),
            "zXbdgDuFe82CwhnxVMfJHhJNGT78MWXELf53BRnwXFps".to_string(),
            "ubFr1VZmfc4zkRQJYm1Mx74mcHzLoDy1QLvxeA5JG9rX".to_string(),
            "v1v4cBXP9L7M9ryZZCx7tuXuNb9pnDLGb3JJkPBpbR1Z".to_string(),
            "vVr7RJ2h83Y243ijt5mNc7mp4LRuvVHZXaKaa8sfDBXD".to_string(),
            "29TdwP6mJQYGd31CmMwiWtR2ZnQBiv3ncgU9LY83sC7bV".to_string(),
            "23pqRUb4qvUzRL7cixE57zo7zT2NZK5wzrr6WtmUMFZ43".to_string(),
        ];
        let params = SubscribeParams::Event(EventSubscribeParams {
            addresses,
            event_index: None,
        });
        subscribe(params).await
    }

    #[tokio::test]
    async fn test_subscribe_block() {
        subscribe(SubscribeParams::Simple(SimpleEventType::Block)).await
    }

    #[tokio::test]
    async fn test_subscribe_tx() {
        subscribe(SubscribeParams::Simple(SimpleEventType::Tx)).await
    }
}
