use crate::{
    common::card::{AuctionType, Card, CardColor},
    components::card_deck::CardDeckView,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="sanitize.css"/>
        <Stylesheet id="leptos-1" href="main.css"/>
        <Stylesheet id="leptos" href="uno.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let cards = vec![
        Card {
            color: CardColor::Purple,
            ty: AuctionType::Free,
            id: 1,
        },
        Card {
            color: CardColor::Blue,
            ty: AuctionType::Fist,
            id: 2,
        },
        Card {
            color: CardColor::Red,
            ty: AuctionType::Circle,
            id: 3,
        },
        Card {
            color: CardColor::Yellow,
            ty: AuctionType::Marked,
            id: 4,
        },
        Card {
            color: CardColor::Green,
            ty: AuctionType::Double,
            id: 5,
        },
    ];
    let (cards, set_cards) = create_signal(cards);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div class="bg-gray w-full p-4">
            <CardDeckView cards=cards/>
        </div>
    }
}

// TODO: better responsive design
