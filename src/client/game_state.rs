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

pub(crate) type MoneySplit = [Money; 6];
pub(crate) const MONEY_VALUE: MoneySplit = [100, 50, 20, 10, 5, 1];

pub fn split_money(input: Money) -> MoneySplit {
    let mut input = input;
    let mut result = [0 as Money; 6];

    for (i, value) in MONEY_VALUE.iter().enumerate() {
        result[i] = input / value;
        input %= value;
    }

    result
}
