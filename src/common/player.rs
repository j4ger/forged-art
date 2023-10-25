use super::{card::Card, game_state::Money};

pub(crate) type PlayerID = u8;

pub(crate) struct Player {
    pub(crate) uuid: String,
    pub(crate) id: PlayerID,
    pub(crate) cards: Vec<Card>,
    pub(crate) money: Money,
    // TODO: statistics
}
