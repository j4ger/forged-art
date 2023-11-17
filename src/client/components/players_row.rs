use crate::client::components::ident_icon::IdentIconView;
use crate::common::{
    game_state::{AuctionState, GameStage, GameState},
    player::Player,
};
use leptos::*;

#[component]
pub fn PlayersRowView(#[prop(into)] state: Signal<GameState>) -> impl IntoView {
    let players = move || {
        let game_state = state();
        let result: Vec<(Player, bool)> = game_state
            .players
            .into_iter()
            .map(|player| {
                let active = match &game_state.stage {
                    GameStage::WaitingForNextCard(next) => player.id == *next,
                    GameStage::WaitingForDoubleTarget { current, .. } => player.id == *current,
                    GameStage::WaitingForMarkedPrice { starter, .. } => player.id == *starter,
                    GameStage::AuctionInAction { state, .. } => match state.get_state() {
                        AuctionState::Free { .. } => true,
                        AuctionState::Circle { current_player, .. } => player.id == *current_player,
                        AuctionState::Fist { .. } => true,
                        AuctionState::Marked { current_player, .. } => player.id == *current_player,
                        _ => unreachable!(),
                    },
                };
                (player, active)
            })
            .collect();
        result
    };
    view! {
        <div class="mt-2">
            <For
                each=players
                key=|(player,_)| player.id
                let:pair
            >
                <div class="inline-block text-center mx-2">
                    <div class=("animation-hithere", pair.1)>
                        <IdentIconView name=pair.0.name.clone()/>
                    </div>
                    <span>{pair.0.name}</span>
                </div>
            </For>
        </div>
    }
}
