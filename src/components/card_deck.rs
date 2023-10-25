use crate::common::card::Card;
use crate::components::card::CardView;
use leptos::*;

#[component]
pub(crate) fn CardDeckView(cards: ReadSignal<Vec<Card>>) -> impl IntoView {
    view! {
        <div class="h-full display-grid grid-cols-6">
            <For
                each=cards
                key=|card| card.id
                let:card
            >
                <CardView card=card/>
            </For>
        </div>
    }
}
