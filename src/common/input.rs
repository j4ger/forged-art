use super::game_state::Money;

pub(crate) type CardID = usize;

pub(crate) const GAME_WS_URL: &'static str = "/ws/game";

#[derive(Debug, Clone, Copy, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) enum GameInput {
    Action(ActionInput),
    RequestState,
    Connect,
    Disconnect,
}

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

