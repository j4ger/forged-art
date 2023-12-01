use crate::client::components::color_count::ColorCountView;
use crate::common::{card::CardColor, player::Player};
use leptos::*;

#[component]
pub fn PlayerInfoView(#[prop(into)] player: Signal<Player>) -> impl IntoView {
    let name = player.get_untracked().name;
    let color_counts = [(); 5]
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let color = CardColor::from_index(i);
            let count = Signal::derive(move || {
                player()
                    .owned_cards
                    .iter()
                    .filter(|card| card.color == color)
                    .count()
            });
            (color, count)
        })
        .collect::<Vec<(CardColor, Signal<usize>)>>()
        .try_into()
        .unwrap();
    let card_count = Signal::derive(move || player().owned_cards.len());
    view! {
        <div class="px-4 flex-inline flex-items-center">
            <span>{name}</span>
            <ColorCountView color_counts/>
            <span class="varela ml-2">"Total: " {card_count}</span>
        </div>
    }
}
