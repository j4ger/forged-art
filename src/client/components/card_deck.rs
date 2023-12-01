use crate::client::components::card::CardView;
use crate::common::card::Card;
use leptos::*;

#[component]
pub(crate) fn CardDeckView(
    #[prop(into)] cards: Signal<Vec<Card>>,
    #[prop(optional)] selectable: bool,
) -> impl IntoView {
    let wrapper_class = "inline-block ml--10
        rotate-5 hover:rotate-0 active:rotate-0 transition-transform-500
        relative z-0 hover:z-1 active:z-1
        translate-y-0 hover:translate-y--5 active:translate-y--5";
    view! {
        <figure class="mt--8 pt-12 pb-2 pl-18 pr-8 nowrap overflow-y-hidden">
            <For each=cards key=|card| card.id let:card>
                <div class=wrapper_class>
                    <CardView card selectable/>
                </div>
            </For>
        </figure>
    }
}
