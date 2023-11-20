use crate::common::card::{AuctionType, Card, CardColor};
use leptos::ev::DragEvent;
use leptos::*;

pub const CARD_ID_FORMAT: &'static str = "mart/card";

#[component]
pub(crate) fn CardView(
    card: Card,
    #[prop(optional)] selectable: bool,
    #[prop(optional)] display_only: bool,
) -> impl IntoView {
    let selected_card: RwSignal<Option<Card>> = use_context().unwrap();
    let selected =
        Signal::derive(move || selected_card().is_some_and(|current| current.id == card.id));

    let dragging: RwSignal<bool> = use_context().unwrap();
    let wrapper_class = format!(
        "{} {}
        w-40 h-50 rd-2 border-3 border-solid transition-all relative select-none
        overflow-hidden shadow-xl scale-100
        hover:shadow-2xl active:shadow-2xl hover:scale-110 active:scale-110
        animation-fall",
        card.color.comp_bg(),
        card.color.main_bd()
    );
    let ty_bg_class = format!(
        "{}
        h-10 flex flex-justify-center flex-items-center shadow-lg",
        card.color.main_bg()
    );
    let ty_fg_class = format!(
        "{}
        novcento text-md font-bold",
        card.color.comp_fg()
    );
    let on_dragstart = move |ev: DragEvent| {
        ev.data_transfer()
            .unwrap()
            .set_data(CARD_ID_FORMAT, card.id.to_string().as_str())
            .ok();
        dragging.set(true);
    };
    let on_dragend = move |_| {
        dragging.set(false);
    };
    let on_click = move |_| {
        if selectable {
            if selected() {
                selected_card.set(None);
            } else {
                selected_card.set(Some(card));
            }
        }
    };

    view! {
        <div
            class=("cursor-grabbing", move || selectable && !selected())
            class=("cursor-context-menu", selected)
            class=("glow", move || selected() && !display_only)
            class=wrapper_class
            prop:draggable=selectable
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            on:click=on_click
        >
            <div class="flex flex-justify-center">
                <img src="abstract.jpg" class="aspect-square w-40 h-40 pointer-events-none"/>
            </div>
            <div
                class=ty_bg_class
            >
                <span
                    class=ty_fg_class
                >
                {card.ty.text()}
                </span>
            </div>
        </div>
    }
}

impl AuctionType {
    pub(crate) fn text(&self) -> &'static str {
        match self {
            Self::Free => "Free",
            Self::Circle => "Circle",
            Self::Fist => "Fist",
            Self::Marked => "Marked",
            Self::Double => "Double",
        }
    }
}

impl CardColor {
    pub(crate) fn comp_fg(&self) -> &'static str {
        match self {
            Self::Red | Self::Blue | Self::Purple => "c-white",
            Self::Green | Self::Yellow => "c-black",
        }
    }

    pub(crate) fn comp_bg(&self) -> &'static str {
        match self {
            Self::Red | Self::Blue | Self::Purple => "bg-white",
            Self::Green | Self::Yellow => "bg-black",
        }
    }

    pub(crate) fn main_fg(&self) -> &'static str {
        match self {
            Self::Red => "c-red-700",
            Self::Green => "c-green",
            Self::Blue => "c-blue",
            Self::Purple => "c-purple",
            Self::Yellow => "c-yellow",
        }
    }

    pub(crate) fn main_bg(&self) -> &'static str {
        match self {
            Self::Red => "bg-red-600",
            Self::Green => "bg-green",
            Self::Blue => "bg-blue",
            Self::Purple => "bg-purple",
            Self::Yellow => "bg-yellow",
        }
    }

    pub(crate) fn main_bd(&self) -> &'static str {
        match self {
            Self::Red => "border-red-600",
            Self::Green => "border-green",
            Self::Blue => "border-blue",
            Self::Purple => "border-purple",
            Self::Yellow => "border-yellow",
        }
    }
}
