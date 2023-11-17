use crate::client::components::{card_deck::CardDeckView, player_info::PlayerInfoView};
use crate::common::player::Player;
use leptos::*;

#[component]
pub(crate) fn CardListView(#[prop(into)] player: Signal<Player>) -> impl IntoView {
    let cards = Signal::derive(move || player.get().owned_cards);
    view! {
        <details open>
            <summary>
                <PlayerInfoView player/>
            </summary>
            <CardDeckView cards selectable=false />
        </details>
    }
}
