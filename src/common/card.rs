// Copy is implemented for card
// so maybe we should only fetch card-image by its id
// rather than assigning a string for each card since it may be copied
#[derive(Clone, Copy)]
pub(crate) struct Card {
    pub(crate) color: CardColor,
    pub(crate) ty: AuctionType,
    pub(crate) id: usize,
}

#[derive(Clone, Copy)]
pub(crate) enum AuctionType {
    Free,
    Circle,
    Fist,
    Marked,
    Double,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum CardColor {
    Red,    // fg: White
    Green,  // fg: Black
    Blue,   // fg: White
    Purple, // fg: White
    Yellow, // fg: Black
}

impl CardColor {
    pub(crate) fn index(&self) -> usize {
        match self {
            Self::Red => 0,
            Self::Green => 1,
            Self::Blue => 2,
            Self::Purple => 3,
            Self::Yellow => 4,
        }
    }

    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Purple,
            4 => Self::Yellow,
            _ => unreachable!(),
        }
    }
}
