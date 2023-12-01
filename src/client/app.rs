use super::{context::inject_game_context, websocket::WsInner};
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

        <Title text="Forged Art"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
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
    inject_game_context();
    let game_state: RwSignal<GameState> = expect_context();
    let player: RwSignal<Player> = expect_context();

    let (count, set_count) = create_signal(0);
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
        <button class="mb-4" on:click=on_click>
            "Click Me: "
            {count}
        </button>

        <ActionPanelView/>

        <CardListView player/>
        <PlayerHandView/>
    }
}

// TODO: better responsive design
// TODO: reduce unnecessary divs
// TODO: history
// TODO: use_web_notification
// TODO: clean up unwraps
// TODO: fix warnings prompted in console.log
// TODO: use leptos_animation for text animation

// TODO: probably need to refactor action panel so that views for all states are statically generated
// only their visibility is controlled by <Show>

