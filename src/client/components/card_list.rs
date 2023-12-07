use crate::client::components::{card_deck::CardDeckView, player_info::PlayerInfoView};
use crate::common::game_state::GameState;
use crate::common::player::Player;
use leptos::*;

#[component]
pub(crate) fn CardListView(#[prop(into)] player: Signal<Player>) -> impl IntoView {
    let game_state: RwSignal<GameState> = expect_context();
    let cards = Signal::derive(move || game_state().owned_cards[player.get_untracked().id].clone());
    view! {
        <details open>
            <summary>
                <PlayerInfoView player/>
            </summary>
            <CardDeckView cards selectable=false/>
        </details>
    }
}

