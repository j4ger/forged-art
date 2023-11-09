use crate::common::{
    card::Card,
    game_state::{GameState, Money},
};

impl GameState {
    pub(self) fn get_deck(&self) -> &Vec<Card> {
        self.deck.get(0).unwrap()
    }

    pub(self) fn get_money(&self) -> &Money {
        self.money.get(0).unwrap()
    }
}
