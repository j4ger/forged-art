use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

use super::{
    card::{Card, CardColor},
    input::ActionInput,
    player::{Player, PlayerID},
};

pub(crate) type Money = f32;
pub(crate) type TimeSpan = Duration;
pub(crate) type Timestamp = DateTime<Utc>;

pub(crate) struct GameState {
    pub(crate) stage: GameStage,
    pub(crate) players: Vec<Player>,
    pub(crate) rounds_passed: u8,
}

pub(crate) enum GameStage {
    WaitingForCard(PlayerID),
    Bidding(BidType),
}

pub(crate) enum BidType {
    Free {
        highest: (Money, Player),
        time_end: Timestamp,
    },
}

impl GameState {
    pub(crate) fn process_input(&mut self, user: &Player, input: ActionInput) -> Result<()> {
        Ok(())
    }

    fn round_should_end(&self) -> bool {
        let mut counters = vec![0u32; 5];
        for player in self.players.iter() {
            for card in player.cards.iter() {
                if let Some(count) = counters.get_mut(card.color as usize) {
                    *count += 1;
                }
            }
        }

        todo!()
    }
}
