use super::card::Card;

pub(crate) type PlayerID = usize;

#[derive(Clone)]
pub(crate) struct Player {
    pub(crate) uuid: String,
    pub(crate) id: PlayerID,
    pub(crate) name: String,
    pub(crate) owned_cards: Vec<Card>,
    // TODO: statistics
}
