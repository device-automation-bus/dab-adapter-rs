use crate::dab::structs::DabError;
use crate::hw_specific::configuration::get_ip_address;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

pub async fn ws_open() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, DabError> {
    let rdk_address = format!("ws://{}:9998/jsonrpc", get_ip_address());
    let url = Url::parse(&rdk_address).expect("Invalid WebSocket URL");

    connect_async(url)
        .await
        .map_err(|e| DabError::Err500(format!("Failed to connect: {}", e)))
        .map(|(ws_stream, _)| ws_stream)
}

pub async fn ws_close(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<(), DabError> {
    ws_stream
        .close(None)
        .await
        .map_err(|e| DabError::Err500(format!("Failed to close WebSocket: {}", e)))
}

pub async fn ws_send(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    payload: Value,
) -> Result<(), DabError> {
    ws_stream
        .send(Message::Text(payload.to_string()))
        .await
        .map_err(|e| DabError::Err500(format!("Failed to send message: {}", e)))
}

pub async fn ws_receive(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<Value, DabError> {
    match timeout(Duration::from_secs(10), ws_stream.next()).await {
        Ok(Some(Ok(message))) => {
            if let Message::Text(text) = message {
                serde_json::from_str(&text)
                    .map_err(|e| DabError::Err500(format!("Invalid JSON: {}", e)))
            } else {
                Err(DabError::Err500("Received a non-text message".to_string()))
            }
        }
        Ok(Some(Err(e))) => Err(DabError::Err500(format!("Error reading message: {:?}", e))),
        Ok(None) => Err(DabError::Err500(
            "The WebSocket stream has been closed by the server".to_string(),
        )),
        Err(_) => Err(DabError::Err500(
            "Timeout occurred while waiting for a response".to_string(),
        )),
    }
}
