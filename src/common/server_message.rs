use super::{
    game_state::{AuctionTarget, GameState, MoneyPair},
    player::PlayerID,
};

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Debug, Clone)]
#[archive(check_bytes)]
pub enum ServerMessage {
    StateUpdate(GameState),
    GameEvent(GameEvent),
    StringMessage(String),
    Disconnect,
    GameStop,
}

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Debug, Clone)]
#[archive(check_bytes)]
pub enum GameEvent {
    PlayerConnect(PlayerID),
    PlayerDisconnect(PlayerID),
    AuctionComplete {
        target: AuctionTarget,
        buyer: MoneyPair,
        seller: PlayerID,
    },
}

