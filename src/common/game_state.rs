use super::{
    card::{Card, CardColor},
    player::{Player, PlayerID},
};
use chrono::{DateTime, Duration, Utc};

pub(crate) type Money = u32;
pub(crate) type TimeSpan = Duration;
pub(crate) type Timestamp = DateTime<Utc>;

#[derive(Clone)]
pub(crate) struct GameState {
    pub(crate) deck: Vec<Vec<Card>>,
    pub(crate) money: Vec<Money>,
    pub(crate) players: Vec<Player>,
    pub(crate) stage: GameStage,
    pub(crate) current_round: usize,
    pub(crate) values: [[Money; 5]; 5],
}

#[derive(Clone)]
pub(crate) enum GameStage {
    WaitingForNextCard(PlayerID),
    WaitingForDoubleTarget {
        double_card: Card,
        starter: PlayerID,
        current: PlayerID,
    },
    WaitingForMarkedPrice {
        marked_card: Card,
        starter: PlayerID,
        double: Option<CardPair>,
    },
    AuctionInAction {
        state: AuctionState,
        target: AuctionTarget,
    },
}

pub(crate) type CardPair = (PlayerID, Card);
pub(crate) type MoneyPair = (PlayerID, Money);

#[derive(Clone, Copy)]
pub(crate) enum AuctionTarget {
    Single(CardPair),
    Double {
        double_card: CardPair,
        target_card: CardPair,
    },
}

#[derive(Clone)]
pub(crate) enum AuctionState {
    Free {
        host: PlayerID,
        highest: MoneyPair,
        time_end: Timestamp,
        calls: u8,
    },
    Circle {
        starter: PlayerID,
        current_player: PlayerID,
        highest: MoneyPair,
    },
    Fist {
        bids: Vec<Money>,
        can_end: bool,
    },
    Marked {
        starter: PlayerID,
        current_player: PlayerID,
        price: Money,
    },
    Double {
        target: Box<AuctionState>,
    },
}

#[derive(Clone, Copy)]
pub(crate) enum ShouldEnd {
    Yes(CardColor),
    No,
}

impl GameState {
    pub(crate) fn round_should_end(&self) -> ShouldEnd {
        let mut counters = vec![0u32; 5];
        for player in self.players.iter() {
            for card in player.owned_cards.iter() {
                if let Some(count) = counters.get_mut(card.color.index()) {
                    *count += 1;
                }
            }
        }

        if let Some(position) = counters.iter().position(|x| x == &5) {
            let color = CardColor::from_index(position);
            ShouldEnd::Yes(color)
        } else {
            ShouldEnd::No
        }
    }
}

impl AuctionState {
    pub(crate) fn get_state(&self) -> &AuctionState {
        match self {
            AuctionState::Double { target } => target.as_ref(),
            _ => &self,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            money: vec![11, 4, 51, 4, 19],
            deck: vec![vec![]; 5],
            players: vec![
                Player {
                    uuid: 1.to_string(),
                    id: 0,
                    name: "Player0".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Red,
                        ty: super::card::AuctionType::Free,
                        id: 0,
                    }],
                },
                Player {
                    uuid: 2.to_string(),
                    id: 1,
                    name: "Player1".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Green,
                        ty: super::card::AuctionType::Circle,
                        id: 1,
                    }],
                },
                Player {
                    uuid: 3.to_string(),
                    id: 2,
                    name: "Player2".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Blue,
                        ty: super::card::AuctionType::Fist,
                        id: 2,
                    }],
                },
                Player {
                    uuid: 4.to_string(),
                    id: 3,
                    name: "Player3".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Purple,
                        ty: super::card::AuctionType::Double,
                        id: 3,
                    }],
                },
                Player {
                    uuid: 5.to_string(),
                    id: 4,
                    name: "Player4".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Yellow,
                        ty: super::card::AuctionType::Marked,
                        id: 4,
                    }],
                },
            ],
            stage: GameStage::WaitingForNextCard(0),
            current_round: 0,
            values: [
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
            ],
        }
    }
}
