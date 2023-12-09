use crate::{
    client::{
        components::{
            action_panel::{ActionPanelView, AuctionTargetView},
            ident_icon::PlayerIconView,
            money_display::MoneyDisplayView,
            player_hand::PlayerHandView,
        },
        context::inject_game_context,
    },
    common::{game_state::GameState, server_message::GameEvent},
};
use leptos::*;
use leptos_icons::{
    BiIcon::{BiArrowToRightSolid, BiBankSolid, BiNoSignalRegular, BiSignal5Regular},
    Icon,
};

#[derive(Clone)]
pub struct EventModal {
    event: Option<GameEvent>,
}

impl EventModal {
    fn new() -> RwSignal<Self> {
        RwSignal::new(Self { event: None })
    }

    pub fn show(&mut self, event: GameEvent) {
        self.event = Some(event);
    }
}

#[component]
pub fn InGameView() -> impl IntoView {
    let modal = EventModal::new();
    provide_context(modal);

    inject_game_context();

    view! {
        <Portal>
            <dialog prop:open=move || modal().event.is_some()>
                <article>
                    <header>
                        <a
                            class="close"
                            on:click=move |_| { modal.update(|modal| modal.event = None) }
                        ></a>
                        <p>"Notification"</p>
                    </header>
                    {if let Some(event) = modal().event {
                        match event {
                            GameEvent::PlayerDisconnect(player_id) => {
                                view! {
                                    <div class="flex flex-justify-center flex-items-center">
                                        <PlayerIconView id=player_id/>
                                        <Icon
                                            icon=Icon::from(BiNoSignalRegular)
                                            width="50px"
                                            height="50px"
                                            class="c-red"
                                        />
                                    </div>
                                }
                                    .into_view()
                            }
                            GameEvent::PlayerConnect(player_id) => {
                                view! {
                                    <div class="flex flex-justify-center flex-items-center">
                                        <PlayerIconView id=player_id/>
                                        <Icon
                                            icon=Icon::from(BiSignal5Regular)
                                            width="50px"
                                            height="50px"
                                            class="c-green"
                                        />
                                    </div>
                                }
                                    .into_view()
                            }
                            GameEvent::AuctionComplete { target, buyer: (buyer, price), seller } => {
                                view! {
                                    <div class="flex flex-col flex-justify-center flex-items-center">
                                        <AuctionTargetView target/>
                                        <div class="flex flex-justify-center flex-items-center">
                                            <PlayerIconView id=buyer/>
                                            <div class="flex flex-col flex-justify-center flex-items-center">
                                                <MoneyDisplayView value=price/>
                                                <Icon
                                                    icon=Icon::from(BiArrowToRightSolid)
                                                    width="40px"
                                                    height="40px"
                                                />
                                            </div>
                                            {if buyer == seller {
                                                view! {
                                                    <Icon
                                                        icon=Icon::from(BiBankSolid)
                                                        height="60px"
                                                        width="60px"
                                                    />
                                                }
                                                    .into_view()
                                            } else {
                                                view! { <PlayerIconView id=seller/> }.into_view()
                                            }}

                                        </div>
                                    </div>
                                }
                                    .into_view()
                            }
                            GameEvent::RoundEnd => {
                                view! { <h3 class="text-center">"Round has ended."</h3> }
                                    .into_view()
                            }
                            GameEvent::GameEnd => {
                                view! { <h3 class="text-center">"Game has ended."</h3> }.into_view()
                            }
                        }
                    } else {
                        ().into_view()
                    }}

                    <footer>
                        <button on:click=move |_| {
                            modal.update(|modal| modal.event = None)
                        }>"OK"</button>
                    </footer>
                </article>
            </dialog>
        </Portal>

        <ActionPanelView/>
        <PlayerHandView/>
    }
}

