use leptos::*;

use crate::server::identicon::get_identicon;

#[component]
pub fn IdentIconView(name: String) -> impl IntoView {
    let icon_data = create_resource(
        move || name.clone(),
        |name| async move { get_identicon(name).await },
    );
    let icon_src = move || icon_data.and_then(|data| format!("data:image/png;base64,{}", data));
    let icon = move || {
        icon_src().map(|src| match src {
            Err(_) => {
                view! {
                    <div>
                        <span>"err"</span>
                    </div>
                }
            }
            Ok(src) => {
                view! {
                    <div class="rd-50% pa-2 bg-white inline-block">
                        <img class="rd-3" prop:src=src/>
                    </div>
                }
            }
        })
    };

    view! {
        // TODO: icon while loading
        <Suspense
            fallback=move || view! { <span>"loading"</span> }
        >
            {icon}
        </Suspense>
    }
}
