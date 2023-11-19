use crate::client::components::card_landing::CardLandingView;
use crate::common::card::Card;
use crate::common::{
    game_state::{GameStage, GameState},
    player::Player,
};
use leptos::*;

#[component]
pub fn ActionPanelView() -> impl IntoView {
    let state: RwSignal<GameState> = use_context().unwrap();
    let player: RwSignal<Player> = use_context().unwrap();
    let selected_card: RwSignal<Option<Card>> = use_context().unwrap();

    let not_selected = move || selected_card().is_none();

    let active = move || state().stage.is_player_active(player().id);

    let (message, content, action) = match state().stage {
        GameStage::WaitingForNextCard(_) => {
            if active() {
                (
                    "Select next auction item.",
                    view! {
                        <div class="flex flex-justify-center">
                            <CardLandingView/>
                        </div>
                    }
                    .into_view(),
                    view! {
                        <button
                            class="mb-0"
                            prop:disabled=move || selected_card().is_none()
                        >"Confirm"</button>
                    }
                    .into_view(),
                )
            } else {
                (
                    "Waiting for next auction item.",
                    ().into_view(),
                    ().into_view(),
                )
            }
        }
        _ => todo!(),
    };

    view! {
        <article>
            <header class="text-center">
                <span>{message}</span>
            </header>
            <div class="container">
                {content}
            </div>
            <footer>
                <div class="container">
                    {action}
                </div>
            </footer>
        </article>
    }
}
