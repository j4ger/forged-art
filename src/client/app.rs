use crate::{
    client::{
        components::{
            card_list::CardListView, global_info::GlobalInfoView, money_input::MoneyInputView,
            money_pile::MoneyPileView, players_row::PlayersRowView,
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

    let auto_animate = r#"
        import autoAnimate from "https://cdn.jsdelivr.net/npm/@formkit/auto-animate@0.8.1/index.min.js";
        document.querySelectorAll(".animate").forEach(function(item) {
            autoAnimate(item);
        });
    "#;

    view! {
        <Stylesheet href="sanitize.css"/>
        <Stylesheet id="leptos-1" href="main.css"/>
        <Stylesheet id="leptos" href="uno.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>

        <script type="module" inner_html=auto_animate>
        </script>

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
    let game_state = RwSignal::new(GameState::default());

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
    ];
    let player = RwSignal::new(Player {
        uuid: "114".into(),
        id: 0,
        name: "Player1".into(),
        owned_cards: cards,
    });
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

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <CardListView player=player/>
        <GlobalInfoView state=game_state/>
        <PlayersRowView state=game_state/>
        <MoneyInputView set_result=set_money max=max/>
    }
}

// TODO: better responsive design
// TODO: reduce unnecessary divs
