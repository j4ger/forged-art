use crate::client::components::card_deck::CardDeckView;
use crate::common::game_state::GameState;
use leptos::*;

#[component]
pub fn PlayerHandView() -> impl IntoView {
    let game_state: RwSignal<GameState> = use_context().unwrap();
    let deck = Signal::derive(move || game_state().deck[0].clone());

    view! {
        <div class="
            fixed bottom--50 w-100% select-none
            hover:bottom--5 active:bottom--5
            opacity-80 hover:opacity-100 active:opacity-100 transition-property-all transition-500"
        >
            <div class="absolute top-8 w-96% h-100% shadow-xl bg-comp rd-t-5 left-0 right-0 mx-a"></div>
            <div class="w-100% flex flex-justify-center">
                <CardDeckView cards=deck selectable=true/>
            </div>
        </div>
    }
}
