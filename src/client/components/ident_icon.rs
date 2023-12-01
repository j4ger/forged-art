use leptos::*;

use crate::{
    common::{game_state::GameState, player::PlayerID},
    server::identicon::get_identicon,
};

#[component]
pub fn IdentIconView(#[prop(into)] name: MaybeSignal<String>) -> impl IntoView {
    let icon_data = create_resource(
        move || name(),
        |name| async move { get_identicon(name).await },
    );
    let icon_src = move || icon_data.and_then(|data| format!("data:image/png;base64,{}", data));
    let icon = move || {
        icon_src().map(|src| match src {
            Err(_) => {
                view! {
                    <div>
                        <span>"err"</span>
                    </div>
                }
            }
            Ok(src) => {
                view! {
                    <div class="rd-50% pa-2 bg-white inline-block">
                        <img class="rd-3" prop:src=src/>
                    </div>
                }
            }
        })
    };

    view! { <Suspense fallback=move || view! { <span>"loading"</span> }>{icon}</Suspense> }
}

#[component]
pub fn PlayerIconView(
    #[prop(into)] id: MaybeSignal<PlayerID>,
    #[prop(optional)] active: bool,
) -> impl IntoView {
    let game_state: RwSignal<GameState> = expect_context();
    let name = Signal::derive(move || game_state.get_untracked().players[id()].name.clone());

    view! {
        <div class="inline" class=("animation-hithere", active)>
            <IdentIconView name/>
        </div>
    }
}

