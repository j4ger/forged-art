use crate::client::components::money_display::{Action, MoneyDisplayView};
use crate::client::game_state::{split_money, MONEY_VALUE};
use crate::common::game_state::Money;
use leptos::*;

const MARGIN: [u32; 6] = [2, 3, 3, 3, 4, 4];

#[component]
pub fn MoneyPileView(#[prop(into)] value: Signal<Money>) -> impl IntoView {
    let values = move || split_money(value());
    let piles = move || {
        values()
            .iter()
            .enumerate()
            .map(|(i, count)| {
                let value = MONEY_VALUE[i];
                let mut inner = Vec::new();
                for _ in 0..*count {
                    inner.push(view! {
                        <MoneyDisplayView value action=Action::Decrease/>
                    });
                }
                let class = format!("inline-flex flex-col-reverse mr--{}", MARGIN[i]);
                view! {
                    <div class=class>
                        {inner}
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <div class="flex flex-items-end flex-justify-center h-min-10">
            {piles}
        </div>
    }
}
