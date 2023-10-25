use super::{card::Card, game_state::Money};

pub(crate) enum ActionInput {
    PlayCard(Card),
    Bet(Money),
    BetOptional(BetOptionalInner),
    SelectDouble(Card),
}

pub(crate) enum BetOptionalInner {
    GiveUp,
    Bet(Money),
}
