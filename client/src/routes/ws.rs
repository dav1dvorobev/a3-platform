use a3_transport::{Receiver, Sender, message::Address};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use std::str::FromStr;

#[derive(serde::Deserialize)]
struct ClientMessage {
    to: Address,
    body: String,
}

pub async fn ws(upgrade: WebSocketUpgrade) -> impl IntoResponse {
    upgrade.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let Some(address) = read_address(&mut socket).await else {
        return;
    };
    let (sender, mut receiver) = match a3_transport::nats::connect(address).await {
        Ok(transport) => transport,
        Err(_) => return,
    };
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut client_to_transport = tokio::spawn(async move {
        while let Some(Ok(Message::Text(payload))) = ws_receiver.next().await {
            if let Ok(message) = serde_json::from_str::<ClientMessage>(payload.as_str()) {
                let _ = sender.send(message.to, message.body).await;
            } else {
                break;
            }
        }
    });
    let mut transport_to_client = tokio::spawn(async move {
        while let Ok(Some(message)) = receiver.recv().await {
            let Ok(payload) = serde_json::to_string(&message) else {
                continue;
            };
            if ws_sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut client_to_transport => transport_to_client.abort(),
        _ = &mut transport_to_client => client_to_transport.abort(),
    };
}

async fn read_address(socket: &mut WebSocket) -> Option<Address> {
    match socket.recv().await {
        Some(Ok(Message::Text(payload))) => Address::from_str(payload.trim()).ok(),
        _ => None,
    }
}
