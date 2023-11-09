use crate::common::card::{AuctionType, Card, CardColor};
use leptos::*;

#[component]
pub(crate) fn CardView(card: Card, #[prop(optional)] selectable: bool) -> impl IntoView {
    let (selectable, _) = create_signal(selectable);
    view! {
        <div class="w-40 h-50 rd-2 border-2 transition-all
            shadow-xl z-0
            hover:shadow-2xl hover:z-1 hover:rotate-0 hover:scale-110"
            class=("cursor-grabbing", selectable)
            class=card.color.comp_bg()
            prop:draggable=selectable
        >
            <div class="flex flex-justify-center">
                <img src="abstract.jpg" class="aspect-square w-40 h-40 rd-t-2 pointer-events-none" />
            </div>
            <div class="h-10 rd-b-2 flex flex-justify-center flex-items-center shadow-lg" class=card.color.main_bg()>
                <span class="text-md font-bold" class=card.color.comp_fg()>{card.ty.text()}</span>
            </div>
        </div>
    }
}

impl AuctionType {
    fn text(&self) -> &'static str {
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
    fn comp_fg(&self) -> &'static str {
        match self {
            Self::Red | Self::Blue | Self::Purple => "c-white",
            Self::Green | Self::Yellow => "c-black",
        }
    }

    fn comp_bg(&self) -> &'static str {
        match self {
            Self::Red | Self::Blue | Self::Purple => "bg-white",
            Self::Green | Self::Yellow => "bg-black",
        }
    }

    fn main_fg(&self) -> &'static str {
        match self {
            Self::Red => "c-red-700",
            Self::Green => "c-green",
            Self::Blue => "c-blue",
            Self::Purple => "c-purple",
            Self::Yellow => "c-yellow",
        }
    }

    fn main_bg(&self) -> &'static str {
        match self {
            Self::Red => "bg-red-600",
            Self::Green => "bg-green",
            Self::Blue => "bg-blue",
            Self::Purple => "bg-purple",
            Self::Yellow => "bg-yellow",
        }
    }

    fn main_bd(&self) -> &'static str {
        match self {
            Self::Red => "border-red-600",
            Self::Green => "border-green",
            Self::Blue => "border-blue",
            Self::Purple => "border-white",
            Self::Yellow => "border-yellow",
        }
    }
}
