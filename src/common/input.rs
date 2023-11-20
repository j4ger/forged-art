use super::game_state::Money;

pub(crate) type CardID = usize;

#[derive(Debug, Clone, Copy, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) enum ActionInput {
    PlayCard(CardID),
    PlayCardOptional(PlayCardOptionalInner),
    Bid(Money),
    BidOptional(BidOptionalInner),
    MarkedReaction(MarkedReactionInner),
    AssignMarkedPrice(Money),
    Call,
}

#[derive(Debug, Clone, Copy, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) enum MarkedReactionInner {
    Accept,
    Pass,
}

#[derive(Debug, Clone, Copy, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) enum BidOptionalInner {
    Pass,
    Bid(Money),
}

#[derive(Debug, Clone, Copy, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) enum PlayCardOptionalInner {
    Pass,
    Play(CardID),
}
