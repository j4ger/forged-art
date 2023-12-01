use super::{
    card::Card,
    game_state::{AuctionTarget, CardPair, Money, MoneyPair},
    player::PlayerID,
};

pub trait PlaceHolder {
    fn placeholder() -> Self;
}

impl PlaceHolder for Card {
    fn placeholder() -> Self {
        Self {
            id: 0,
            color: super::card::CardColor::Red,
            ty: super::card::AuctionType::Free,
        }
    }
}

impl PlaceHolder for PlayerID {
    fn placeholder() -> Self {
        0
    }
}

impl PlaceHolder for Money {
    fn placeholder() -> Self {
        0
    }
}

impl PlaceHolder for CardPair {
    fn placeholder() -> Self {
        (PlayerID::placeholder(), Card::placeholder())
    }
}

impl PlaceHolder for MoneyPair {
    fn placeholder() -> Self {
        (PlayerID::placeholder(), Money::placeholder())
    }
}

impl PlaceHolder for AuctionTarget {
    fn placeholder() -> Self {
        Self::Single(CardPair::placeholder())
    }
}

