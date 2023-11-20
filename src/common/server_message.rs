use super::{game_state::GameState, player::PlayerID};

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub enum ServerMessage {
    StateUpdate(GameState),
    GameEvent(GameEvent),
}

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub enum GameEvent {
    PlayerConnect(PlayerID),
    PlayerDisconnect(PlayerID),
}
