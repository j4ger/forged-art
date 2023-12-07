use crate::{
    client::{
        components::{
            action_panel::ActionPanelView, card_list::CardListView, player_hand::PlayerHandView,
        },
        context::inject_game_context,
    },
    common::{
        card::{AuctionType, Card, CardColor},
        game_state::{GameState, Money},
        player::Player,
    },
};
use leptos::*;

#[component]
pub fn InGameView() -> impl IntoView {
    inject_game_context();

    view! {
        <ActionPanelView/>
        <PlayerHandView/>
    }
}

