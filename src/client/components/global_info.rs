use crate::client::components::color_count::ColorCountView;
use crate::common::card::CardColor;
use crate::common::game_state::GameState;
use leptos::*;

#[component]
pub fn GlobalInfoView() -> impl IntoView {
    let game_state: RwSignal<GameState> = expect_context();
    let round = move || game_state().current_round;
    let color_counts = [(); 5]
        .iter()
        .enumerate()
        .map(|(i, ())| {
            let color = CardColor::from_index(i);
            let count = Signal::derive(move || {
                game_state()
                    .owned_cards
                    .iter()
                    .map(move |owned_cards| {
                        owned_cards
                            .iter()
                            .filter(|card| card.color == color)
                            .count()
                    })
                    .reduce(|x, y| x + y)
                    .unwrap()
            });
            (color, count)
        })
        .collect::<Vec<(CardColor, Signal<usize>)>>()
        .try_into()
        .unwrap(); // TODO: find a way to refactor this pile of cringe

    let table = move || {
        let mut sum = [0u32; 5];
        let table_body = game_state()
            .values
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let inner = row
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        sum[i] += item;
                        let color = CardColor::from_index(i);
                        let outer_class = color.main_bg();
                        let inner_class = format!("{} varela", color.comp_fg());
                        let content = if *item == 0 {
                            "".into()
                        } else {
                            item.to_string()
                        };
                        view! {
                            <td class=outer_class>
                                <span class=inner_class>{content}</span>
                            </td>
                        }
                    })
                    .collect_view();
                view! {
                    <tr>
                        <th scope="row">
                            <span class="varela">{i}</span>
                        </th>
                        {inner}
                    </tr>
                }
            })
            .collect_view();

        let table_footer = sum
            .iter()
            .enumerate()
            .map(|(i, sum)| {
                let color = CardColor::from_index(i);
                let outer_class = color.main_bg();
                let inner_class = format!("{} varela", color.comp_fg());
                view! {
                    <td scope="col" class=outer_class>
                        <span class=inner_class>{*sum}</span>
                    </td>
                }
            })
            .collect_view();

        view! {
            <table>
                <tbody>{table_body}</tbody>
                <tfoot>
                    <tr>
                        <th scope="col">
                            <span>"Sum"</span>
                        </th>
                        {table_footer}
                    </tr>
                </tfoot>
            </table>
        }
    };

    view! {
        <article class="container">
            <div>
                <span class="ml-2">"Round:"</span>
                // TODO: icons
                <div class="ml-2">
                    <span class="varela">{round}</span>
                    <span>/</span>
                    <span class="varela">"4"</span>
                </div>
            </div>
            <div>
                <span class="ml-2">"Counts:"</span>
                <ColorCountView color_counts=color_counts/>
            </div>
            <div class="max-w-40">
                <table>{table}</table>
            </div>
        </article>
    }
}

