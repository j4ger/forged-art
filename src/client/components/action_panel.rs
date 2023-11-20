use crate::client::components::{
    card::CardView, card_landing::CardLandingView, ident_icon::PlayerIconView,
    money_input::MoneyInputView,
};
use crate::client::websocket::Ws;
use crate::common::game_state::Money;
use crate::common::{
    card::Card,
    game_state::{GameStage, GameState},
    input::*,
    player::{Player, PlayerID},
};
use leptos::*;

#[component]
pub fn ActionPanelView() -> impl IntoView {
    let game_state: RwSignal<GameState> = use_context().unwrap();
    let player: RwSignal<Player> = use_context().unwrap();
    let selected_card: RwSignal<Option<Card>> = use_context().unwrap();
    let ws: Ws = use_context().unwrap();

    let not_selected = move || selected_card().is_none();

    let active = move || game_state().stage.is_player_active(player().id);

    let inner = move || {
        let (message, content, action) = match game_state().stage {
            GameStage::WaitingForNextCard(_) => {
                if active() {
                    (
                        "Select next auction item.",
                        view! { <CardLandingView/> }.into_view(),
                        view! {
                            <button
                                class="mb-0"
                                prop:disabled=not_selected
                                on:click=move |_| {
                                    ws
                                        .get_value()
                                        .send_game_input(
                                            ActionInput::PlayCard(selected_card().unwrap().id),
                                        )
                                }
                            >
                                "Confirm"
                            </button>
                        }
                        .into_view(),
                    )
                } else {
                    (
                        "Waiting for next auction item.",
                        ().into_view(),
                        ().into_view(),
                    )
                }
            }
            GameStage::WaitingForDoubleTarget {
                double_card,
                starter,
                ..
            } => {
                if active() {
                    (
                        "Select next auction item.",
                        view! {
                            <DoubleCardView double_card player_id=starter/>
                            <div class="flex flex-items-center flex-col">
                                <PlayerIconView id=player.get_untracked().id active=true/>
                                <span>"Your choice:"</span>
                                <CardLandingView/>
                            </div>
                        }
                        .into_view(),
                        view! {
                            <button
                                class="mb-0 w-40%"
                                prop:disabled=not_selected
                                on:click=move |_| {
                                    ws
                                        .get_value()
                                        .send_game_input(
                                            ActionInput::PlayCardOptional(
                                                PlayCardOptionalInner::Play(selected_card().unwrap().id),
                                            ),
                                        )
                                }
                            >
                                "Confirm"
                            </button>
                            <button
                                class="mb-0 w-40% secondary"
                                on:click=move |_| {
                                    ws
                                        .get_value()
                                        .send_game_input(
                                            ActionInput::PlayCardOptional(PlayCardOptionalInner::Pass),
                                        )
                                }
                            >
                                "Skip"
                            </button>
                        }
                        .into_view(),
                    )
                } else {
                    (
                        "Waiting for next aution item.",
                        view! {
                            <DoubleCardView double_card player_id=starter/>
                        }
                        .into_view(),
                        ().into_view(),
                    )
                }
            }
            GameStage::WaitingForMarkedPrice {
                marked_card,
                starter,
                double,
            } => {
                let double_card_view = double.map(|(player_id, double_card)| {
                    view! {
                        <DoubleCardView double_card player_id/>
                    }
                });
                if active() {
                    let (price, set_price) = create_signal(0 as Money);
                    (
                        "Input marked price.",
                        view! {
                            {double_card_view}
                            <PlayerCardView card=marked_card player_id=starter active=true message="Current card:"/>
                            <MoneyInputView set_result=set_price/>
                        }
                        .into_view(),
                        view!{
                            <button
                                on:click=move |_| ws.get_value().send_game_input(
                                    ActionInput::AssignMarkedPrice(price())
                                )
                            >"Confirm"</button>
                        }.into_view()
                    )
                } else {
                    (
                        "Waiting for setting marked price.",
                        view! {
                            {double_card_view}
                            <PlayerCardView card=marked_card player_id=starter active=false message="Current card:"/>
                        }.into_view(),
                        ().into_view()
                    )
                }
            }
            _ => todo!(),
        };
        view! {
            <header class="text-center">
                <span>{message}</span>
            </header>
            <div class="container flex flex-justify-center flex-items-center gap-2">{content}</div>
            <footer>
                <div class="flex flex-justify-center gap-2">{action}</div>
            </footer>
        }
    };

    view! { <article>{inner}</article> }
}

#[component]
fn DoubleCardView(double_card: Card, player_id: PlayerID) -> impl IntoView {
    view! {
        <PlayerCardView card=double_card player_id active=false message="Double Card:"/>
    }
}

#[component]
fn PlayerCardView(
    card: Card,
    player_id: PlayerID,
    active: bool,
    message: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-items-center flex-col">
            <PlayerIconView id=player_id active/>
            <span>{message}</span>
            <CardView card/>
        </div>
    }
}
