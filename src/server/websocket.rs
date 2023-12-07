use crate::common::{input::GameInput, player::PlayerID, server_message::ServerMessage};
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use rkyv::{from_bytes, to_bytes};
use tokio::sync::{broadcast, mpsc};

pub struct GameInfo {
    pub players: Vec<(String, String)>, // (uuid, name)
    pub read: broadcast::Sender<(Option<PlayerID>, ServerMessage)>,
    pub write: mpsc::UnboundedSender<(PlayerID, GameInput)>,
}

pub static GAME_INFO_STORE: Lazy<DashMap<String, GameInfo>> = Lazy::new(|| DashMap::new());

pub async fn game_websocket(
    Path(game_id): Path<String>,
    Path(uuid): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    if let Some(inner) = GAME_INFO_STORE.get(&game_id) {
        if let Some(player_id) = inner.players.iter().position(|player| player.0 == uuid) {
            ws.on_upgrade(move |ws| handle_game_websocket(ws, game_id, player_id))
                .into_response()
        } else {
            Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("You are not allowed to join this game."))
                .unwrap()
                .into_response()
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Invalid Game ID. "))
            .unwrap()
            .into_response()
    }
}

async fn handle_game_websocket(socket: WebSocket, game_id: String, player_id: PlayerID) {
    let (mut sender, mut receiver) = socket.split();

    let game_info = GAME_INFO_STORE.get(&game_id).unwrap();
    let mut read = game_info.read.subscribe();
    let write = game_info.write.clone();

    write.send((player_id, GameInput::Connect)).unwrap();

    let mut write_task = tokio::spawn(async move {
        while let Some(Ok(Message::Binary(data))) = receiver.next().await {
            if let Ok(input) = from_bytes::<GameInput>(&data) {
                write.send((player_id, input)).unwrap();
            }
        }
    });

    let mut read_task = tokio::spawn(async move {
        while let Ok((target, message)) = read.recv().await {
            if target.is_none() | target.is_some_and(|id| id == player_id) {
                let data = to_bytes::<_, 4>(&message).unwrap().to_vec();
                if sender.send(Message::Binary(data)).await.is_err() {
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut write_task) => read_task.abort(),
        _ = (&mut read_task) => write_task.abort()
    };

    game_info
        .write
        .send((player_id, GameInput::Disconnect))
        .unwrap();
}

