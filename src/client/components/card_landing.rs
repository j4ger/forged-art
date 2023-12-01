use crate::client::components::card::{CardView, CARD_ID_FORMAT};
use crate::common::card::Card;
use crate::common::game_state::GameState;
use leptos::ev::DragEvent;
use leptos::*;

#[component]
pub fn CardLandingView() -> impl IntoView {
    let game_state: RwSignal<GameState> = expect_context();
    let selected_card: RwSignal<Option<Card>> = expect_context();
    let dragging: RwSignal<bool> = expect_context();

    let get_card = move |ev: DragEvent| {
        let card_id: usize = ev
            .data_transfer()
            .unwrap()
            .get_data(CARD_ID_FORMAT)
            .unwrap()
            .parse()
            .unwrap();
        let card = game_state
            .get_untracked()
            .deck
            .get(0)
            .unwrap()
            .iter()
            .filter(|card| card.id == card_id)
            .next()
            .map(|card| *card);
        card
    };

    let (hovering, set_hovering) = create_signal(false);

    let on_dragover = move |ev: DragEvent| {
        ev.prevent_default();
        set_hovering(true);
    };
    let on_dragleave = move |ev: DragEvent| {
        ev.prevent_default();
        set_hovering(false);
    };
    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        let card = get_card(ev);
        selected_card.set(card);
        set_hovering(false);
    };

    let on_click = move |_| {
        selected_card.set(None);
    };

    let inner = move || match selected_card() {
        Some(card) => view! {
            <div on:click=on_click class="cursor-context-menu">
                <CardView card display_only=true/>
            </div>
        }
        .into_view(),
        None => view! {
            <div class="h-60% flex flex-justify-center flex-items-center">
                <span>"drag to here"</span>
            </div>
        }
        .into_view(),
    };

    view! {
        <div
            class="box-content w-40 h-50 border-dashed border-3 rd-3"
            class=("!border-solid", move || hovering() || selected_card().is_some())
            class=("!border-4", dragging)
            class=("!border-5", move || selected_card().is_some() && !hovering())
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
        >
            {inner}
        </div>
    }
}

