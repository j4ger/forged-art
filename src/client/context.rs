use crate::client::websocket::WsInner;
use crate::common::card::{AuctionType, Card, CardColor};
use crate::common::game_state::{AuctionTarget, GameStage};
use crate::common::player::Player;
use crate::common::{game_state::GameState, server_message::ServerMessage};
use leptos::*;

pub fn inject_game_context() {
    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let game_state = RwSignal::new(GameState::default());
    provide_context(game_state);
    // use it with:
    // let game_state: RwSignal<GameState> = expect_context();

    let cards = vec![
        Card {
            color: CardColor::Purple,
            ty: AuctionType::Free,
            id: 1,
        },
        Card {
            color: CardColor::Blue,
            ty: AuctionType::Fist,
            id: 2,
        },
        Card {
            color: CardColor::Red,
            ty: AuctionType::Circle,
            id: 3,
        },
        Card {
            color: CardColor::Yellow,
            ty: AuctionType::Marked,
            id: 4,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 5,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 6,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 7,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 8,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 9,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 10,
        },
    ];
    let player = RwSignal::new(Player {
        uuid: "114".into(),
        id: 0,
        name: "Player1".into(),
        owned_cards: cards,
        connected: true,
    });
    provide_context(player);
    // use it with:
    // let player: RwSignal<Player> = expect_context();

    let balance = Signal::derive(move || game_state().money[player().id]);
    provide_context(balance);
    // use it with:
    // let balance: RwSignal<Money> = expect_context();

    let ws = WsInner::new("/api/game");
    ws.set_onmessage(move |message| match message {
        ServerMessage::StateUpdate(state) => {
            game_state.set(state);
        }
        ServerMessage::GameEvent(event) => {
            todo!()
        }
    });
    let ws = store_value(ws);
    provide_context(ws);
    // use it with:
    // let ws: RwSignal<Ws> = expect_context();

    let dragging: RwSignal<bool> = RwSignal::new(false);
    provide_context(dragging);
    // use it with:
    // let dragging: RwSignal<bool> = expect_context();
}

// TODO: remove this after test
impl Default for GameState {
    fn default() -> Self {
        let dummy_card1 = Card {
            color: CardColor::Red,
            ty: AuctionType::Free,
            id: 101,
        };
        let dummy_card2 = Card {
            color: CardColor::Red,
            ty: AuctionType::Free,
            id: 101,
        };
        GameState {
            money: vec![514, 4, 51, 4, 19],
            deck: vec![
                vec![
                    Card {
                        color: CardColor::Red,
                        ty: AuctionType::Free,
                        id: 10,
                    },
                    Card {
                        color: CardColor::Green,
                        ty: AuctionType::Circle,
                        id: 11,
                    },
                    Card {
                        color: CardColor::Blue,
                        ty: AuctionType::Marked,
                        id: 12,
                    },
                    Card {
                        color: CardColor::Purple,
                        ty: AuctionType::Double,
                        id: 13,
                    },
                    Card {
                        color: CardColor::Red,
                        ty: AuctionType::Free,
                        id: 14,
                    },
                    Card {
                        color: CardColor::Green,
                        ty: AuctionType::Circle,
                        id: 15,
                    },
                    Card {
                        color: CardColor::Blue,
                        ty: AuctionType::Marked,
                        id: 16,
                    },
                    Card {
                        color: CardColor::Purple,
                        ty: AuctionType::Double,
                        id: 17,
                    },
                ],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            players: vec![
                Player {
                    uuid: 1.to_string(),
                    id: 0,
                    name: "Player0".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Red,
                        ty: AuctionType::Free,
                        id: 0,
                    }],
                    connected: true,
                },
                Player {
                    uuid: 2.to_string(),
                    id: 1,
                    name: "Player1".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Green,
                        ty: AuctionType::Circle,
                        id: 1,
                    }],
                    connected: true,
                },
                Player {
                    uuid: 3.to_string(),
                    id: 2,
                    name: "Player2".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Blue,
                        ty: AuctionType::Fist,
                        id: 2,
                    }],
                    connected: true,
                },
                Player {
                    uuid: 4.to_string(),
                    id: 3,
                    name: "Player3".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Purple,
                        ty: AuctionType::Double,
                        id: 3,
                    }],
                    connected: true,
                },
                Player {
                    uuid: 5.to_string(),
                    id: 4,
                    name: "Player4".into(),
                    owned_cards: vec![Card {
                        color: CardColor::Yellow,
                        ty: AuctionType::Marked,
                        id: 4,
                    }],
                    connected: true,
                },
            ],
            // stage: GameStage::WaitingForDoubleTarget {
            //     double_card: dummy_card1,
            //     starter: 1,
            //     current: 0,
            // },
            // stage: GameStage::WaitingForNextCard(0),
            // stage: GameStage::WaitingForDoubleTarget {
            //     double_card: (0, dummy_card1),
            //     current: 0,
            // },
            stage: GameStage::WaitingForMarkedPrice {
                starter: 0,
                target: AuctionTarget::Double {
                    double_card: (1, dummy_card2),
                    target_card: (0, dummy_card1),
                },
            },
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

