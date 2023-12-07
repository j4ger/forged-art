use crate::client::{
    components::in_game::InGameView,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="sanitize.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>
        <Stylesheet id="leptos" href="uno.css"/>
        <Stylesheet id="leptos-1" href="main.css"/>

        <Title text="Forged Art"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=InGameView/>
                </Routes>
            </main>
        </Router>
    }
}

// TODO: better responsive design
// TODO: reduce unnecessary divs
// TODO: history
// TODO: use_web_notification
// TODO: clean up unwraps
// TODO: fix warnings prompted in console.log
// TODO: use leptos_animation for text animation
// TODO: hijack right-click menu

