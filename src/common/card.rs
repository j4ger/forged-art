// Copy is implemented for card
// so maybe we should only fetch card-image by its id
// rather than assigning a string for each card since it may be copied
#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[archive(check_bytes)]
pub(crate) struct Card {
    pub(crate) color: CardColor,
    pub(crate) ty: AuctionType,
    pub(crate) id: usize, // globally unique in a game, should start at 1
}

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[archive(check_bytes)]
pub(crate) enum AuctionType {
    Free,
    Circle,
    Fist,
    Marked,
    Double,
}

#[derive(
    rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Debug, Clone, Copy, Eq, PartialEq, Hash,
)]
#[archive(check_bytes)]
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
            Self::Red => 2,
            Self::Green => 3,
            Self::Blue => 1,
            Self::Purple => 0,
            Self::Yellow => 4,
        }
    }

    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            2 => Self::Red,
            3 => Self::Green,
            1 => Self::Blue,
            0 => Self::Purple,
            4 => Self::Yellow,
            _ => unreachable!(),
        }
    }
}

