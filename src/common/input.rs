use super::game_state::Money;

pub(crate) type CardID = usize;

#[derive(Clone, Copy)]
pub(crate) enum ActionInput {
    PlayCard(CardID),
    PlayCardOptional(PlayCardOptionalInner),
    Bid(Money),
    BidOptional(BidOptionalInner),
    MarkedReaction(MarkedReactionInner),
    AssignMarkedPrice(Money),
    Call,
}

#[derive(Clone, Copy)]
pub(crate) enum MarkedReactionInner {
    Accept,
    Pass,
}

#[derive(Clone, Copy)]
pub(crate) enum BidOptionalInner {
    Pass,
    Bid(Money),
}

#[derive(Clone, Copy)]
pub(crate) enum PlayCardOptionalInner {
    Pass,
    Play(CardID),
}
