use crate::common::card::CardColor;
use leptos::*;

#[component]
pub fn ColorCountView(color_counts: [(CardColor, Signal<usize>); 5]) -> impl IntoView {
    let color_stats = color_counts
        .into_iter()
        .map(|(color, count)| {
            let wrapper_class = format!(
                "{} 
                w-4 h-4 rd-1 flex flex-justify-center flex-items-center",
                color.main_bg()
            );
            let text_class = format!(
                "{}
                varela font-size-3",
                color.comp_fg()
            );
            view! {
                <div class=wrapper_class>
                    <span class=text_class>{count}</span>
                </div>
            }
        })
        .collect_view();
    view! {
        <div class="ml-2 flex gap-1 flex-items-center">
            {color_stats}
        </div>
    }
}
