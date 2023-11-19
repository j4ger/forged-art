use crate::client::components::ident_icon::IdentIconView;
use crate::common::{game_state::GameState, player::Player};
use leptos::*;

#[component]
pub fn PlayersRowView() -> impl IntoView {
    let state: RwSignal<GameState> = use_context().unwrap();
    let players = move || {
        let game_state = state();
        let result: Vec<(Player, bool)> = game_state
            .players
            .into_iter()
            .map(|player| {
                let active = game_state.stage.is_player_active(player.id);
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
