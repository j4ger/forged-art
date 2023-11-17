use crate::client::components::card::CardView;
use crate::common::card::Card;
use leptos::*;

#[component]
pub(crate) fn CardDeckView(
    #[prop(into)] cards: Signal<Vec<Card>>,
    #[prop(optional)] selectable: bool,
) -> impl IntoView {
    let wrapper_class = if selectable {
        ""
    } else {
        "inline-block ml--10 rotate-5 hover:rotate-0 z-0 hover:z-1 transition-all"
    };
    view! {
        <figure class="py-3 px-14 pr-4 overflow-auto nowrap">
                <For
                    each=cards
                    key=|card| card.id
                    let:card
                >
                    <div class=wrapper_class>
                        <CardView card selectable/>
                    </div>
                </For>
        </figure>
    }
}
