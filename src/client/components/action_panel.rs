use crate::client::components::{
    card::CardView, card_landing::CardLandingView, ident_icon::PlayerIconView,
    money_input::MoneyInputView,
};
use crate::client::websocket::Ws;
use crate::common::game_state::{AuctionState, AuctionTarget, CardPair, Money, MoneyPair};
use crate::common::placeholder::PlaceHolder;
use crate::common::{
    card::Card,
    game_state::{GameStage, GameState},
    input::*,
    player::{Player, PlayerID},
};
use leptos::*;
use leptos_icons::{BiIcon::BiMoneyRegular, Icon};
use leptos_use::{use_timestamp_with_controls, UseTimestampReturn};

macro_rules! match_or {
    ($target:expr, $pattern:pat,$transform:expr,$fallback:expr) => {
        match $target {
            $pattern => $transform,
            _ => $fallback,
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum SubView {
    WaitingForCard,
    WaitingForDoubleTarget,
    WaitingForMarkedPrice,
    FreeAuction,
    MarkedAuction,
    FistAuction,
    CircleAuction,
}

#[component]
pub fn ActionPanelView() -> impl IntoView {
    let game_state: RwSignal<GameState> = expect_context();
    let player: Signal<Player> = expect_context();
    let self_id = player.get_untracked().id;
    let active = Signal::derive(move || game_state().stage.is_player_active(self_id));
    provide_context(active);

    let subview = create_memo(move |_| match game_state().stage {
        GameStage::WaitingForNextCard(_) => SubView::WaitingForCard,
        GameStage::WaitingForDoubleTarget { .. } => SubView::WaitingForDoubleTarget,
        GameStage::WaitingForMarkedPrice { .. } => SubView::WaitingForMarkedPrice,
        GameStage::AuctionInAction { state, .. } => match state {
            AuctionState::Free { .. } => SubView::FreeAuction,
            AuctionState::Circle { .. } => SubView::CircleAuction,
            AuctionState::Fist { .. } => SubView::FistAuction,
            AuctionState::Marked { .. } => SubView::MarkedAuction,
        },
    });
    provide_context(subview);

    let selected_card: RwSignal<Option<Card>> = RwSignal::new(None);
    provide_context(selected_card);

    let double_card = create_read_slice(game_state, |state| {
        match_or!(
            state.stage,
            GameStage::WaitingForDoubleTarget { double_card, .. },
            double_card,
            CardPair::placeholder()
        )
    });

    let marked_card = create_read_slice(game_state, |state| {
        match_or!(
            state.stage,
            GameStage::WaitingForMarkedPrice { target, .. },
            target,
            AuctionTarget::placeholder()
        )
    });

    let auction_target = create_read_slice(game_state, |state| {
        match_or!(
            state.stage,
            GameStage::AuctionInAction { target, .. },
            target,
            AuctionTarget::placeholder()
        )
    });

    let host = create_read_slice(game_state, |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(
                state,
                AuctionState::Free { host, .. } | AuctionState::Fist { host, .. },
                *host,
                PlayerID::placeholder()
            ),
            PlayerID::placeholder()
        )
    });

    let free_calls = create_read_slice(game_state, |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(state, AuctionState::Free { calls, .. }, *calls, 0),
            0
        )
    });

    let free_time_end = create_read_slice(game_state, |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(state, AuctionState::Free { time_end, .. }, *time_end, 0.0),
            0.0
        )
    });

    let highest = create_read_slice(game_state, |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(
                state,
                AuctionState::Free { highest, .. } | AuctionState::Circle { highest, .. },
                *highest,
                MoneyPair::placeholder()
            ),
            MoneyPair::placeholder()
        )
    });

    let marked_price = create_read_slice(game_state, |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(
                state,
                AuctionState::Marked { price, .. },
                *price,
                MoneyPair::placeholder()
            ),
            MoneyPair::placeholder()
        )
    });

    let player_count = game_state.get_untracked().players.len();
    let fist_action_taken = create_read_slice(game_state, move |state| {
        match_or!(
            &state.stage,
            GameStage::AuctionInAction { state, .. },
            match_or!(
                state,
                AuctionState::Fist { action_taken, .. },
                action_taken.clone(),
                vec![false; player_count]
            ),
            vec![false; player_count]
        )
    });

    view! {
        <WaitingForCardView/>
        <WaitingForDoubleTargetView double_card/>
        <WaitingForMarkedPriceView target=marked_card/>
        <FreeAuctionView
            target=auction_target
            highest
            host
            calls=free_calls
            time_end=free_time_end
        />
        <MarkedAuctionView target=auction_target price=marked_price/>
        <CircleAuctionView target=auction_target current=highest/>
        <FistAuctionView target=auction_target action_taken=fist_action_taken host/>
    }
}

#[component]
fn WaitingForCardView() -> impl IntoView {
    let selected_card: RwSignal<Option<Card>> = expect_context();
    let ws: Ws = expect_context();

    let not_selected = move || selected_card().is_none();
    view! {
        <Panel
            subview=SubView::WaitingForCard
            active_message="Select next auction item."
            inactive_message="Waiting for next auction item."
        >
            <Active slot>
                <CardLandingView/>
            </Active>
            <Inactive slot>()</Inactive>
            <Action slot>
                <button
                    prop:disabled=not_selected
                    on:click=move |_| {
                        ws.get_value()
                            .send_game_input(ActionInput::PlayCard(selected_card().unwrap().id))
                    }
                >

                    "Confirm"
                </button>
            </Action>
        </Panel>
    }
}

#[component]
fn WaitingForDoubleTargetView(double_card: Signal<CardPair>) -> impl IntoView {
    let selected_card: RwSignal<Option<Card>> = expect_context();
    let ws: Ws = expect_context();

    let not_selected = move || selected_card().is_none();
    let player: Signal<Player> = expect_context();

    view! {
        <Panel
            subview=SubView::WaitingForDoubleTarget
            active_message="Select next auction item."
            inactive_message="Waiting for next auction item."
        >
            <Active slot>
                <DoubleCardView double_card/>
                <div class="flex flex-items-center flex-col">
                    <PlayerIconView id=player.get_untracked().id active=true/>
                    <span>"YOU CHOOSE"</span>
                    <CardLandingView/>
                </div>
            </Active>
            <Inactive slot>
                <DoubleCardView double_card/>
            </Inactive>
            <Action slot>
                <button
                    prop:disabled=not_selected
                    on:click=move |_| {
                        ws.get_value()
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
                    class="secondary"
                    on:click=move |_| {
                        ws.get_value()
                            .send_game_input(
                                ActionInput::PlayCardOptional(PlayCardOptionalInner::Pass),
                            )
                    }
                >

                    "Skip"
                </button>
            </Action>
        </Panel>
    }
}

#[component]
fn WaitingForMarkedPriceView(#[prop(into)] target: Signal<AuctionTarget>) -> impl IntoView {
    let ws: Ws = expect_context();

    let (price, set_price) = create_signal(0 as Money);

    view! {
        <Panel
            subview=SubView::WaitingForMarkedPrice
            active_message="Set marked auction price."
            inactive_message="Waiting for marked auction price to be set."
        >
            <Active slot>
                <AuctionTargetView target/>
                <MoneyInputView set_result=set_price/>
            </Active>
            <Inactive slot>
                <AuctionTargetView target/>
            </Inactive>
            <Action slot>
                <button on:click=move |_| {
                    ws.get_value().send_game_input(ActionInput::AssignMarkedPrice(price()))
                }>

                    "Confirm"
                </button>
            </Action>
        </Panel>
    }
}

#[component]
fn FreeAuctionView(
    target: Signal<AuctionTarget>,
    highest: Signal<MoneyPair>,
    host: Signal<PlayerID>,
    calls: Signal<u8>,
    time_end: Signal<f64>,
) -> impl IntoView {
    let ws: Ws = expect_context();

    let (price, set_price) = create_signal(0 as Money);

    let player: Signal<Player> = expect_context();
    let is_host = Signal::derive(move || player().id == host());

    let (call_disabled, set_call_disabled) = create_signal(false);

    let UseTimestampReturn {
        timestamp,
        pause,
        resume,
        ..
    } = use_timestamp_with_controls();
    let progress_value = Signal::derive(move || {
        let value = time_end() * 1000.0 - timestamp();
        if value > 0.0 {
            set_call_disabled(true);
            resume();
            (3000.0 - value) as i32
        } else {
            set_call_disabled(false);
            pause();
            3000i32
        }
    });

    view! {
        <Panel
            subview=SubView::FreeAuction
            active_message="Free auction in action."
            inactive_message=""
        >
            <Active slot>
                <AuctionTargetView target/>
                <div class="flex flex-col flex-items-center">
                    <div class="flex flex-items-center gap-6">
                        <PriceDisplayView pair=highest/>
                        <div class="flex flex-col p-1 rd-1 b-1 b-solid">
                            <span class="novcento text-center">"Calls"</span>
                            <span
                                class="varela text-center font-size-6 font-800"
                                class=("c-red", move || calls() == 2)
                            >
                                {calls}
                            </span>
                            <progress
                                class="max-w-20 mb-0"
                                max="3000"
                                value=progress_value
                            ></progress>
                        </div>
                    </div>
                    <MoneyInputView set_result=set_price/>
                </div>
            </Active>
            <Inactive slot>()</Inactive>
            <Action slot>
                <button on:click=move |_| {
                    ws.get_value().send_game_input(ActionInput::Bid(price()))
                }>

                    "Make Offer"
                </button>
                <Show when=is_host>
                    <button
                        class="contrast"
                        prop:disabled=call_disabled
                        on:click=move |_| ws.get_value().send_game_input(ActionInput::Call)
                    >
                        "Make a Call"
                    </button>
                </Show>
            </Action>
        </Panel>
    }
}

#[component]
fn MarkedAuctionView(target: Signal<AuctionTarget>, price: Signal<MoneyPair>) -> impl IntoView {
    let ws: Ws = expect_context();

    view! {
        <Panel
            subview=SubView::MarkedAuction
            active_message="Decide if you want to accept this offer of marked auction."
            inactive_message="Waiting for other players' choice on marked auction."
        >
            <Active slot>
                <AuctionTargetView target/>
                <PriceDisplayView pair=price/>
            </Active>
            <Inactive slot>
                <AuctionTargetView target/>
                <PriceDisplayView pair=price/>
            </Inactive>
            <Action slot>
                <button on:click=move |_| {
                    ws.get_value()
                        .send_game_input(ActionInput::MarkedReaction(MarkedReactionInner::Accept))
                }>"Accept"</button>
                <button
                    class="secondary"
                    on:click=move |_| {
                        ws.get_value()
                            .send_game_input(ActionInput::MarkedReaction(MarkedReactionInner::Pass))
                    }
                >

                    "Pass"
                </button>
            </Action>
        </Panel>
    }
}

#[component]
fn CircleAuctionView(target: Signal<AuctionTarget>, current: Signal<MoneyPair>) -> impl IntoView {
    let ws: Ws = expect_context();

    let (price, set_price) = create_signal(0 as Money);
    create_effect(move |_| set_price(current().1));
    view! {
        <Panel
            subview=SubView::CircleAuction
            active_message="Make your offer in this circle auction."
            inactive_message="Waiting for other players' choice in this circle auction"
        >
            <Active slot>
                <AuctionTargetView target/>
                <div class="flex flex-col flex-justify-center">
                    <PriceDisplayView pair=current/>
                    <MoneyInputView set_result=set_price/>
                </div>
            </Active>
            <Inactive slot>
                <AuctionTargetView target/>
            </Inactive>
            <Action slot>
                <button on:click=move |_| {
                    ws.get_value()
                        .send_game_input(ActionInput::BidOptional(BidOptionalInner::Bid(price())))
                }>"Make Offer"</button>
                <button
                    class="secondary"
                    on:click=move |_| {
                        ws.get_value()
                            .send_game_input(ActionInput::BidOptional(BidOptionalInner::Pass))
                    }
                >

                    "Pass"
                </button>
            </Action>
        </Panel>
    }
}

#[component]
fn FistAuctionView(
    target: Signal<AuctionTarget>,
    action_taken: Signal<Vec<bool>>,
    host: Signal<PlayerID>,
) -> impl IntoView {
    let ws: Ws = expect_context();

    let (price, set_price) = create_signal(0 as Money);

    let game_state: RwSignal<GameState> = expect_context();
    let player_status = game_state
        .get_untracked()
        .players
        .iter()
        .enumerate()
        .map(|(i, player)| {
            let active = MaybeSignal::derive(move || !*action_taken().get(i).unwrap());
            view! { <PlayerIconView id=player.id active/> }
        })
        .collect_view();

    let player: Signal<Player> = expect_context();
    let is_host = Signal::derive(move || player.get_untracked().id == host());

    let complete_button_disabled = move || action_taken().contains(&false);

    view! {
        <Panel
            subview=SubView::FistAuction
            active_message="Make your offer in this fist auction."
            inactive_message=""
        >
            <Active slot>
                <AuctionTargetView target/>
                <div class="flex flex-col flex-items-center">
                    <div class="flex gap-3">{player_status.clone()}</div>
                    <MoneyInputView set_result=set_price/>
                </div>
            </Active>
            <Inactive slot>()</Inactive>
            <Action slot>
                <button on:click=move |_| {
                    ws.get_value().send_game_input(ActionInput::Bid(price()))
                }>"Make Offer"</button>
                <Show when=is_host>
                    <button
                        class="contrast"
                        prop:disabled=complete_button_disabled
                        on:click=move |_| ws.get_value().send_game_input(ActionInput::Call)
                    >
                        "Complete Auction"
                    </button>
                </Show>
            </Action>
        </Panel>
    }
}

#[slot]
struct Active {
    children: ChildrenFn,
}

#[slot]
struct Inactive {
    children: ChildrenFn,
}

#[slot]
struct Action {
    children: ChildrenFn,
}

#[component]
fn Panel(
    subview: SubView,
    active_message: &'static str,
    inactive_message: &'static str,
    active: Active,
    inactive: Inactive,
    action: Action,
) -> impl IntoView {
    let player_active: Signal<bool> = expect_context();
    let current_subview: Memo<SubView> = expect_context();

    // TODO: Too cringy to read it twice but can't think of a better way for it to work
    view! {
        <Show when=move || current_subview() == subview>
            <article>
                <header class="text-center">
                    <span>
                        <Show when=player_active fallback=move || inactive_message>
                            {active_message}
                        </Show>
                    </span>
                </header>
                <div class="container flex flex-justify-center flex-items-center gap-2">

                    {
                        let active_content = active.children.clone();
                        let inactive_content = inactive.children.clone();
                        view! {
                            <Show
                                when=player_active
                                fallback=move || { inactive_content.clone()() }
                            >
                                {active_content()}
                            </Show>
                        }
                    }

                </div>
                <footer>
                    <div class="flex flex-justify-center px-2 gap-2">

                        {
                            let action = action.children.clone();
                            view! { <Show when=player_active>{action()}</Show> }
                        }

                    </div>
                </footer>
            </article>
        </Show>
    }
}
// fallback=panel_slot.inactive_content

#[component]
fn AuctionTargetView(target: Signal<AuctionTarget>) -> impl IntoView {
    let single_target = Signal::derive(move || match target() {
        AuctionTarget::Single(inner) => inner,
        AuctionTarget::Double { target_card, .. } => target_card,
    });
    let double_target = Signal::derive(move || match target() {
        AuctionTarget::Single(_) => CardPair::placeholder(),
        AuctionTarget::Double { double_card, .. } => double_card,
    });
    let show_double = move || matches!(target(), AuctionTarget::Double { .. });
    view! {
        <Show when=show_double>
            <DoubleCardView double_card=double_target/>
        </Show>
        <PlayerCardView card=single_target active=false message="CURRENT"/>
    }
}

#[component]
fn DoubleCardView(double_card: Signal<CardPair>) -> impl IntoView {
    view! { <PlayerCardView card=double_card active=false message="Double Card:"/> }
}

#[component]
fn PlayerCardView(card: Signal<CardPair>, active: bool, message: &'static str) -> impl IntoView {
    let id = Signal::derive(move || card().0);
    let card = move || {
        view! { <CardView card=card().1/> }
    };
    view! {
        <div class="flex flex-items-center flex-col">
            <PlayerIconView id active/>
            <span class="novcento">{message}</span>
            {card}
        </div>
    }
}

#[component]
fn PriceDisplayView(#[prop(into)] pair: Signal<MoneyPair>) -> impl IntoView {
    let id = Signal::derive(move || pair().0);
    let value = Signal::derive(move || pair().1);
    // TODO: animation
    view! {
        <div class="flex flex-justify-center flex-items-center">
            <PlayerIconView id/>
            <span class="ml-1 mr-3">":"</span>
            <Icon icon=Icon::from(BiMoneyRegular) width="40px" height="40px"/>
            <span class="ml-1 varela font-size-10">{value}</span>
        </div>
    }
}

