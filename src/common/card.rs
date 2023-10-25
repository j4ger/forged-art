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

#[derive(Clone, Copy)]
pub(crate) enum CardColor {
    Red,    // fg: White
    Green,  // fg: Black
    Blue,   // fg: White
    Purple, // fg: White
    Yellow, // fg: Black
}
