use crate::common::game_state::Money;
use leptos::*;
use leptos_icons::{BiIcon::BiMoneyRegular, Icon};

#[derive(Clone, Copy)]
pub enum Action {
    Increase,
    Decrease,
    None,
}

#[component]
pub fn MoneyDisplayView(
    value: Money,
    #[prop(default=Action::None)] action: Action,
    #[prop(optional, into)] residual: Option<Signal<Money>>,
) -> impl IntoView {
    let setter: WriteSignal<Money> = expect_context();
    let valid = Signal::derive(move || residual.map_or(true, |res| value < res()));
    let on_click = move |_| match action {
        Action::Decrease => setter.update(|count| *count -= value),
        Action::Increase => {
            if valid() {
                setter.update(|count| *count += value)
            }
        }
        Action::None => {}
    };

    view! {
        <div
            class="money fill animation-fall-rotate flex-inline flex-justify-center flex-items-center px-1 b-1 b-double rd-1 mb--5"
            class=("money-valid", valid)
            class=("opacity-50", move || !valid())
            on:click=on_click
        >
            <Icon icon=Icon::from(BiMoneyRegular) width="25px" height="25px"/>
            <span class="select-none varela font-1000 pt-0.5">{value}</span>
        </div>
    }
}

