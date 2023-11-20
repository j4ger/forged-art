use super::websocket::WsInner;
use crate::{
    client::{
        components::{
            action_panel::ActionPanelView, card::CARD_ID_FORMAT, card_landing::CardLandingView,
            card_list::CardListView, global_info::GlobalInfoView, money_input::MoneyInputView,
            money_pile::MoneyPileView, player_hand::PlayerHandView, players_row::PlayersRowView,
        },
        error_template::{AppError, ErrorTemplate},
    },
    common::{
        card::{AuctionType, Card, CardColor},
        game_state::{GameStage, GameState, Money},
        player::Player,
        server_message::ServerMessage,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="sanitize.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>
        <Stylesheet id="leptos" href="uno.css"/>
        <Stylesheet id="leptos-1" href="main.css"/>

        <Title text="Welcome to Leptos"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=InGameView/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn InGameView() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let game_state = RwSignal::new(GameState::default());
    provide_context(game_state);

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

    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let player = RwSignal::new(Player {
        uuid: "114".into(),
        id: 0,
        name: "Player1".into(),
        owned_cards: cards,
        connected: true,
    });
    provide_context(player);

    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let balance = Signal::derive(move || game_state().money[player().id]);
    provide_context(balance);

    let mut next_id = 6;
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        player.update(|x| {
            x.owned_cards.push(Card {
                color: CardColor::Red,
                ty: AuctionType::Free,
                id: next_id,
            })
        });
        game_state.update(|x| {
            x.players[0].owned_cards.push(Card {
                color: CardColor::Red,
                ty: AuctionType::Free,
                id: next_id,
            })
        });
        next_id += 1;
    };

    let (money, set_money) = create_signal(0 as Money);
    let (max, _) = create_signal(1145 as Money);
    create_effect(move |_| {
        log::info!("Money state: {:?}", money());
    });

    let selected_card: RwSignal<Option<Card>> = RwSignal::new(None);
    provide_context(selected_card);

    let dragging: RwSignal<bool> = RwSignal::new(false);
    provide_context(dragging);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <ActionPanelView/>

        <CardListView player/>
        <PlayerHandView/>
    }
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
            // stage: GameStage::WaitingForNextCard(1),
            stage: GameStage::WaitingForMarkedPrice {
                marked_card: dummy_card1,
                starter: 0,
                double: Some((1, dummy_card2)),
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
// TODO: better responsive design
// TODO: reduce unnecessary divs
// TODO: history
// TODO: use_web_notification
// TODO: clean up unwraps
// TODO: fix warnings prompted in console.log
