use crate::client::websocket::WsInner;
use crate::common::card::{AuctionType, Card, CardColor};
use crate::common::game_state::{AuctionState, AuctionTarget, GameStage};
use crate::common::input::GAME_WS_URL;
use crate::common::player::Player;
use crate::common::{game_state::GameState, server_message::ServerMessage};
use crate::server::player::get_new_uuid;
use leptos::*;
use leptos_use::storage::{use_local_storage, JsonCodec};

pub fn get_uuid() -> String {
    let (uuid, set_uuid, _) = use_local_storage::<Option<String>, JsonCodec>("uuid");
    match uuid.get_untracked() {
        Some(uuid) => uuid,
        None => {
            spawn_local(async move {
                let uuid = get_new_uuid().await.unwrap();
                set_uuid(Some(uuid));
            });
            uuid.get_untracked().unwrap()
        }
    }
}

pub fn inject_game_context() {
    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let game_state = RwSignal::new(GameState::dummy());
    provide_context(game_state);
    // use it with:
    // let game_state: RwSignal<GameState> = expect_context();

    let uuid = get_uuid();
    let player_id = game_state
        .get_untracked()
        .players
        .iter()
        .find(|player| player.uuid == uuid)
        .unwrap()
        .id;
    let player = Signal::derive(move || game_state().players[player_id].clone());
    provide_context(player);
    // use it with:
    // let player: Signal<Player> = expect_context();

    let balance = Signal::derive(move || game_state().money[player.get_untracked().id]);
    provide_context(balance);
    // use it with:
    // let balance: Signal<Money> = expect_context();

    let ws = WsInner::new(GAME_WS_URL);
    ws.set_onmessage(move |message| match message {
        ServerMessage::StateUpdate(state) => {
            game_state.set(state);
        }
        ServerMessage::GameEvent(event) => {
            todo!()
        }
        ServerMessage::Disconnect => {
            todo!()
        }
        ServerMessage::GameStop => {
            todo!()
        }
        ServerMessage::StringMessage(message) => {
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
impl GameState {
    fn dummy() -> Self {
        #[cfg(feature = "ssr")]
        let now = 114514 as f64;

        #[cfg(not(feature = "ssr"))]
        let now = js_sys::Date::now() / 1000.0;

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
                    connected: true,
                },
                Player {
                    uuid: 2.to_string(),
                    id: 1,
                    name: "Player1".into(),
                    connected: true,
                },
                Player {
                    uuid: 3.to_string(),
                    id: 2,
                    name: "Player2".into(),
                    connected: true,
                },
                Player {
                    uuid: 4.to_string(),
                    id: 3,
                    name: "Player3".into(),
                    connected: true,
                },
                Player {
                    uuid: 5.to_string(),
                    id: 4,
                    name: "Player4".into(),
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
            // stage: GameStage::WaitingForMarkedPrice {
            //     starter: 0,
            //     target: AuctionTarget::Double {
            //         double_card: (1, dummy_card2),
            //         target_card: (0, dummy_card1),
            //     },
            // },
            // stage: GameStage::AuctionInAction {
            //     state: AuctionState::Free {
            //         host: 1,
            //         highest: (0, 114),
            //         time_end: now + 3f64,
            //         calls: 2,
            //     },
            //     target: AuctionTarget::Single((0, dummy_card1)),
            // },
            // stage: GameStage::AuctionInAction {
            //     state: AuctionState::Marked {
            //         price: (0, 114),
            //         current: 0,
            //     },
            //     target: AuctionTarget::Single((0, dummy_card1)),
            // },
            // stage: GameStage::AuctionInAction {
            //     state: AuctionState::Circle {
            //         starter: 0,
            //         current_player: 0,
            //         highest: (0, 114),
            //     },
            //     target: AuctionTarget::Single((0, dummy_card1)),
            // },
            stage: GameStage::AuctionInAction {
                state: AuctionState::Fist {
                    host: 0,
                    bids: vec![0, 0, 0, 0, 0],
                    action_taken: vec![false, false, false, false, false],
                },
                target: AuctionTarget::Single((0, dummy_card1)),
            },
            current_round: 0,
            values: [
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
                [30, 20, 10, 0, 0],
            ],
            pool: Vec::new(),
            owned_cards: vec![
                vec![Card {
                    color: CardColor::Yellow,
                    ty: AuctionType::Marked,
                    id: 1,
                }],
                vec![Card {
                    color: CardColor::Yellow,
                    ty: AuctionType::Marked,
                    id: 2,
                }],
                vec![Card {
                    color: CardColor::Yellow,
                    ty: AuctionType::Marked,
                    id: 3,
                }],
                vec![Card {
                    color: CardColor::Yellow,
                    ty: AuctionType::Marked,
                    id: 4,
                }],
                vec![Card {
                    color: CardColor::Yellow,
                    ty: AuctionType::Marked,
                    id: 5,
                }],
            ],
        }
    }
}

