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
        game_state::{GameState, Money},
        player::Player,
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
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    // WARN: think twice before changing this type, as many components are
    // relying on the type to fetch from the context API
    let game_state = RwSignal::new(GameState::default());

    provide_context(game_state);

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
        <CardListView player/>
        <CardListView player/>
        <PlayerHandView/>
    }
}

// TODO: better responsive design
// TODO: reduce unnecessary divs
// TODO: history
