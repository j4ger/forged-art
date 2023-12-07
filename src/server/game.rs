use super::websocket::{GameInfo, GAME_INFO_STORE};
use crate::common::{
    game_state::GameState,
    input::GameInput,
    server_message::{GameEvent, ServerMessage},
};
use tokio::sync::{broadcast, mpsc};

pub fn start_game(players: Vec<(String, String)>, game_id: String) {
    force_stop_game(&game_id);
    let mut game_state = GameState::new(players.clone());

    let (mpsc_sender, mut read) = mpsc::unbounded_channel();
    let (write, _) = broadcast::channel(5);

    let write_clone = write.clone();

    GAME_INFO_STORE.insert(
        game_id,
        GameInfo {
            players,
            read: write_clone,
            write: mpsc_sender,
        },
    );

    let read_task = tokio::spawn(async move {
        while let Some((player_id, input)) = read.recv().await {
            match input {
                GameInput::RequestState => {
                    write
                        .send((
                            Some(player_id),
                            ServerMessage::StateUpdate(game_state.clone()),
                        ))
                        .unwrap();
                }
                GameInput::Connect => {
                    if game_state.players[player_id].connected {
                        write
                            .send((Some(player_id), ServerMessage::Disconnect))
                            .unwrap();
                        game_state.players[player_id].connected = false;
                        write
                            .send((
                                None,
                                ServerMessage::GameEvent(GameEvent::PlayerDisconnect(player_id)),
                            ))
                            .unwrap();
                    } else {
                        game_state.players[player_id].connected = true;
                        write
                            .send((
                                None,
                                ServerMessage::GameEvent(GameEvent::PlayerConnect(player_id)),
                            ))
                            .unwrap();
                    }
                }
                GameInput::Disconnect => {
                    game_state.players[player_id].connected = false;
                    write
                        .send((
                            None,
                            ServerMessage::GameEvent(GameEvent::PlayerDisconnect(player_id)),
                        ))
                        .unwrap();
                }
                GameInput::Action(input) => match game_state.process_input(player_id, input) {
                    Ok(inner) => {
                        if let Some(event) = inner {
                            write.send((None, ServerMessage::GameEvent(event))).unwrap();
                        }
                    }
                    Err(message) => {
                        write
                            .send((
                                Some(player_id),
                                ServerMessage::StringMessage(message.to_string()),
                            ))
                            .unwrap();
                    }
                },
            }
        }
    });
}

pub fn force_stop_game(game_id: &str) {
    if let Some((_, game)) = GAME_INFO_STORE.remove(game_id) {
        game.read.send((None, ServerMessage::GameStop)).unwrap();
    }
}

