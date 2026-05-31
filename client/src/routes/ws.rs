use a3_transport::{
    Receiver,
    message::{Address, Message as TransportMessage},
};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

pub async fn ws(upgrade: WebSocketUpgrade) -> impl IntoResponse {
    upgrade.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let Some(address) = read_address(&mut socket).await else {
        let _ = send_ws_error(&mut socket, "invalid user address".to_string()).await;
        return;
    };
    let (sender, mut receiver) = match a3_transport::nats::connect(address).await {
        Ok(transport) => transport,
        Err(error) => {
            let _ =
                send_ws_error(&mut socket, format!("transport connection failed: {error}")).await;
            return;
        }
    };
    if send_ws_ready(&mut socket).await.is_err() {
        return;
    };
    loop {
        tokio::select! {
            message = socket.recv() => {
                match message {
                    Some(Ok(Message::Text(payload))) => {
                        if let Err(error) = send_transport_message(&sender, payload.as_str()).await {
                            let _ = send_ws_error(&mut socket, error.to_string()).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
            message = receiver.recv() => {
                match message {
                    Ok(Some(message)) => {
                        if send_ws_message(&mut socket, message).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
        }
    }
}

async fn read_address(socket: &mut WebSocket) -> Option<Address> {
    match socket.recv().await {
        Some(Ok(Message::Text(payload))) => Address::from_str(payload.trim()).ok(),
        _ => None,
    }
}

async fn send_transport_message(
    sender: &impl a3_transport::Sender,
    payload: &str,
) -> a3_transport::Result<()> {
    let value = serde_json::from_str::<serde_json::Value>(payload)?;
    let to = value.get("to").and_then(serde_json::Value::as_str).ok_or(
        a3_transport::Error::AddressError("missing recipient address"),
    )?;
    let body = value
        .get("body")
        .and_then(serde_json::Value::as_str)
        .ok_or(a3_transport::Error::AddressError("missing message body"))?;
    sender.send(Address::from_str(to)?, body.to_string()).await
}

async fn send_ws_message(
    socket: &mut WebSocket,
    message: TransportMessage,
) -> Result<(), axum::Error> {
    let payload = serde_json::to_string(&message).unwrap_or_else(|_| "{}".to_string());
    socket.send(Message::Text(payload.into())).await
}

async fn send_ws_error(socket: &mut WebSocket, message: String) -> Result<(), axum::Error> {
    let payload = serde_json::json!({
        "type": "error",
        "message": message,
    });
    socket.send(Message::Text(payload.to_string().into())).await
}

async fn send_ws_ready(socket: &mut WebSocket) -> Result<(), axum::Error> {
    let payload = serde_json::json!({
        "type": "ready",
    });
    socket.send(Message::Text(payload.to_string().into())).await
}
