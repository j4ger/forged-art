use crate::common::card::{AuctionType, Card, CardColor};
use leptos::*;

#[component]
pub(crate) fn CardView(card: Card, #[prop(optional)] selectable: bool) -> impl IntoView {
    let (selectable, _) = create_signal(selectable);
    let wrapper_class = format!(
        "{}
        my-3 w-40 h-50 rd-2 border-2 transition-all relative
        shadow-xl z-0 scale-100
        hover:shadow-2xl hover:z-1 hover:scale-110
        animation-fall",
        card.color.comp_bg()
    );
    let ty_bg_class = format!(
        "{}
        h-10 rd-b-2 flex flex-justify-center flex-items-center shadow-lg",
        card.color.main_bg()
    );
    let ty_fg_class = format!(
        "{}
        novcento text-md font-bold",
        card.color.comp_fg()
    );
    view! {
        <div
            class=("cursor-grabbing", selectable)
            class=wrapper_class
            prop:draggable=selectable
        >
            <div class="flex flex-justify-center">
                <img src="abstract.jpg" class="aspect-square w-40 h-40 rd-t-2 pointer-events-none" />
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
            Self::Purple => "border-white",
            Self::Yellow => "border-yellow",
        }
    }
}
