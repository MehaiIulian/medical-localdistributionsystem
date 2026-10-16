use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum SignalingEvent {
    JoinRoom { room_id: String, role: String },
    ChatMessage { sender: String, text: String, timestamp: String },
    WebRtcOffer { sdp: String },
    WebRtcAnswer { sdp: String },
    WebRtcCandidate { candidate: String },
    AiBotStatus { status_text: String },
}

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg_result) = socket.next().await {
        if let Ok(msg) = msg_result {
            match msg {
                Message::Text(text) => {
                    // Echo/broadcast signaling mechanism
                    if let Ok(event) = serde_json::from_str::<SignalingEvent>(&text) {
                        let response_text = serde_json::to_string(&event).unwrap_or_default();
                        if socket.send(Message::Text(response_text)).await.is_err() {
                            break;
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

