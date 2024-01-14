use bytes::{BufMut, BytesMut};
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use std::env;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, protocol::Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

fn create_packet(index: u32, data: &str) -> Vec<u8> {
    let mut buffer = BytesMut::new();
    buffer.put_u32(0x0010 + data.len() as u32);
    buffer.put_u16(0x0010);
    buffer.put_u16(1);
    buffer.put_u32(7);
    buffer.put_u32(index);
    buffer.put(&data.as_bytes()[..]);
    buffer.to_vec()
}

async fn connect_to_server(
    url: Url,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Error> {
    let (ws_stream, _) = connect_async(url).await?;
    Ok(ws_stream)
}

pub async fn ws_client(url: Url) {
    let ws_stream = connect_to_server(url).await.unwrap();
    let (mut write, read) = ws_stream.split();

    let data = json!({
        "roomid": "22412359"
    });
    let data = data.to_string();
    let mut index: u32 = 1;
    write
        .send(Message::Binary(create_packet(index, &data)))
        .await
        .unwrap();
    index + 1;
    tokio::spawn(async move {
        read.for_each(|message| async {
            match message {
                Ok(Message::Text(text)) => {
                    // Handle incoming message here.
                    // You'll probably want to send it to your UI using Tauri.
                    println!("Received a message: {}", text);
                }
                Ok(Message::Binary(binary)) => {
                    println!("Received a binary message: {:?}", binary)
                }
                Ok(Message::Close(_)) => {
                    // Handle close frame here
                }
                Err(e) => {
                    println!("E: {}", e);
                }
                _ => {}
            }
        })
        .await;
    });
}
