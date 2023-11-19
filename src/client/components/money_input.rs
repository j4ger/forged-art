use crate::client::components::{
    money_display::{Action, MoneyDisplayView},
    money_pile::MoneyPileView,
};
use crate::client::game_state::MONEY_VALUE;
use crate::common::game_state::Money;
use leptos::*;

#[component]
pub fn MoneyInputView(set_result: WriteSignal<Money>) -> impl IntoView {
    let max: Signal<Money> = use_context().unwrap();
    let (value, set_value) = create_signal(0 as Money);
    let residual = Signal::derive(move || max() - value());
    create_effect(move |_| set_result(value()));

    provide_context(set_value);

    let icon_inputs = MONEY_VALUE
        .map(|value| {
            view! {
                <MoneyDisplayView value action=Action::Increase residual/>
            }
        })
        .collect_view();

    // TODO: test on phone for keypad
    view! {
        <article class="container">
            <div class="mb-4 text-center">
                <span>"max: "{max}</span>
            </div>
            <div class="flex flex-justify-center">
                {icon_inputs}
            </div>
            <div class="flex flex-items-center mt-10 mb-5 mx-2">
                <input
                    class="!mb-0"
                    type="number" name="money" placeholder="Input Money"
                    on:input=move |event| {
                        let str = event_target_value(&event);
                        match str.parse::<Money>() {
                            Ok(result) => {
                                if result > max() {
                                    set_value(max());
                                } else {
                                    set_value(result);
                                }
                            }
                            Err(_) => set_value(0)
                        }
                    }
                    prop:value=value
                    prop:max=max
                    prop:min=0
                />
                <a role="button" href="#" class="h-fit"
                    on:click=move |_| set_value(0)
                >"Reset"</a>
            </div>
            <MoneyPileView value/>
        </article>
    }
}
