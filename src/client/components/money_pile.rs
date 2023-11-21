use crate::client::components::money_display::{Action, MoneyDisplayView};
use crate::client::game_state::{split_money, MONEY_VALUE};
use crate::common::game_state::Money;
use leptos::*;

const MARGIN: [u32; 6] = [2, 3, 3, 3, 4, 4];

#[component]
pub fn MoneyPileView(#[prop(into)] value: Signal<Money>) -> impl IntoView {
    let values = move || split_money(value());
    let piles = (0usize..6usize)
        .map(|i| {
            let value = MONEY_VALUE[i];
            let count = move || 0..values()[i];
            let class = format!("inline-flex flex-col-reverse mr--{}", MARGIN[i]);
            view! {
                <div class=class>
                    <For
                        each=count
                        key=|i| *i
                        let:_
                    >
                        <MoneyDisplayView value action=Action::Decrease/>
                    </For>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="flex flex-items-end flex-justify-center h-min-5 mb-5">
            {piles}
        </div>
    }
}
