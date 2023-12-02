use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};

use crate::common::{
    card::{AuctionType, Card},
    game_state::{AuctionState, AuctionTarget, GameStage, GameState, Money, ShouldEnd},
    input::{ActionInput, BidOptionalInner, CardID, MarkedReactionInner, PlayCardOptionalInner},
    player::{Player, PlayerID},
};

impl GameState {
    pub(self) fn mask(&self, player: PlayerID) -> GameState {
        // TODO: mask stage(fist) money
        GameState {
            deck: vec![self.deck.get(player).unwrap().clone()],
            money: vec![self.money.get(player).unwrap().clone()],
            players: self.players.clone(),
            stage: self.stage.clone(),
            current_round: self.current_round,
            values: self.values,
        }
    }

    pub(self) fn get_next_player_rounded(&self, current: PlayerID) -> PlayerID {
        let mut next = current + 1;
        if next > self.players.len() - 1 {
            next = 0;
        }
        next
    }

    pub(self) fn get_next_player(&self, starter: PlayerID, current: PlayerID) -> Option<PlayerID> {
        let next = self.get_next_player_rounded(current);
        if next == starter {
            return None;
        } else {
            return Some(next);
        }
    }

    fn test_enough_money(&self, player: PlayerID, money: Money) -> Result<()> {
        if self.money.get(player).unwrap() > &money {
            Ok(())
        } else {
            bail!("Not enough money.");
        }
    }

    fn get_card(&self, player_id: PlayerID, card_id: CardID) -> Result<&Card> {
        self.deck
            .get(player_id)
            .unwrap()
            .iter()
            .filter(|card| card.id == card_id)
            .next()
            .context("No such card.")
    }

    fn complete_transaction(
        &mut self,
        target: AuctionTarget,
        money: Money,
        current: PlayerID,
    ) -> GameStage {
        *self.money.get_mut(current).unwrap() -= money;
        let player = self.players.get_mut(current).unwrap();
        let owner;
        match target {
            AuctionTarget::Single((starter, card)) => {
                player.owned_cards.push(card);
                owner = starter;
            }
            AuctionTarget::Double {
                double_card: (_, double_card),
                target_card: (starter, target_card),
            } => {
                player.owned_cards.push(double_card);
                player.owned_cards.push(target_card);
                owner = starter;
            }
        }
        if owner != current {
            *self.money.get_mut(owner).unwrap() += money;
        }
        let next = self.get_next_player_rounded(owner);
        GameStage::WaitingForNextCard(next)
    }

    pub(self) fn process_input(&mut self, from: &mut Player, input: ActionInput) -> Result<()> {
        let next_stage = match (&self.stage, input) {
            (GameStage::WaitingForNextCard(player_id), ActionInput::PlayCard(card_id)) => {
                if *player_id == from.id {
                    let card = play_card(&mut self.deck, from, card_id)?;
                    if let ShouldEnd::Yes(_) = self.round_should_end() {
                        // TODO: end round && clean up
                        return Ok(());
                    }

                    if let AuctionType::Double = card.ty {
                        let next = self.get_next_player_rounded(from.id);
                        GameStage::WaitingForDoubleTarget {
                            double_card: (from.id, card),
                            current: next,
                        }
                    } else if let AuctionType::Marked = card.ty {
                        GameStage::WaitingForMarkedPrice {
                            starter: from.id,
                            target: AuctionTarget::Single((from.id, card)),
                        }
                    } else {
                        let state = self.gen_auction_state(&card, from.id, self.players.len());
                        GameStage::AuctionInAction {
                            state,
                            target: AuctionTarget::Single((from.id, card)),
                        }
                    }
                } else {
                    bail!("Not your turn yet.");
                }
            }
            (
                GameStage::WaitingForDoubleTarget {
                    double_card,
                    current,
                },
                ActionInput::PlayCardOptional(inner),
            ) => {
                if *current == from.id {
                    match inner {
                        PlayCardOptionalInner::Pass => {
                            match self.get_next_player(double_card.0, *current) {
                                Some(next) => GameStage::WaitingForDoubleTarget {
                                    double_card: *double_card,
                                    current: next,
                                },
                                None => {
                                    //TODO: announce free get
                                    let next = self.get_next_player_rounded(from.id);
                                    GameStage::WaitingForNextCard(next)
                                }
                            }
                        }
                        PlayCardOptionalInner::Play(card_id) => {
                            let card = self.get_card(from.id, card_id)?;
                            if card.color != double_card.1.color {
                                bail!("Wrong card color.");
                            }
                            if let AuctionType::Double = card.ty {
                                bail!("Cannot set another double card as the target to a previous double card.");
                            }
                            let card = play_card(&mut self.deck, from, card_id)?;
                            if let ShouldEnd::Yes(_) = self.round_should_end() {
                                // TODO: end round && clean up
                                return Ok(());
                            }
                            if let AuctionType::Marked = card.ty {
                                GameStage::WaitingForMarkedPrice {
                                    starter: from.id,
                                    target: AuctionTarget::Double {
                                        double_card: *double_card,
                                        target_card: (from.id, card),
                                    },
                                }
                            } else {
                                let state =
                                    self.gen_auction_state(&card, from.id, self.players.len());
                                GameStage::AuctionInAction {
                                    state,
                                    target: AuctionTarget::Double {
                                        double_card: *double_card,
                                        target_card: (from.id, card),
                                    },
                                }
                            }
                        }
                    }
                } else {
                    bail!("Not your turn yet.");
                }
            }
            (
                GameStage::WaitingForMarkedPrice { starter, target },
                ActionInput::AssignMarkedPrice(money),
            ) => {
                if *starter == from.id {
                    self.test_enough_money(from.id, money)?;
                    let next = self.get_next_player_rounded(from.id);
                    GameStage::AuctionInAction {
                        state: AuctionState::Marked {
                            current: next,
                            price: (from.id, money),
                        },
                        target: *target,
                    }
                } else {
                    bail!("Not your turn yet.");
                }
            }
            (GameStage::AuctionInAction { state, target }, ActionInput::Bid(money)) => {
                self.test_enough_money(from.id, money)?;
                match state {
                    AuctionState::Free { host, highest, .. } => {
                        if money > highest.1 {
                            let highest = (from.id, money);
                            let calls = 0;
                            let now = SystemTime::now();
                            let time_end = now + Duration::from_secs(3);
                            GameStage::AuctionInAction {
                                state: AuctionState::Free {
                                    host: *host,
                                    highest,
                                    time_end: time_end
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs_f64(),
                                    calls,
                                },
                                target: *target,
                            }
                        } else {
                            bail!("The current price is higher than your offer.");
                        }
                    }
                    AuctionState::Fist {
                        host,
                        bids,
                        action_taken,
                    } => {
                        let mut bids = bids.clone();
                        *bids.get_mut(from.id).unwrap() = money;
                        let mut action_taken = action_taken.clone();
                        *action_taken.get_mut(from.id).unwrap() = true;
                        GameStage::AuctionInAction {
                            state: AuctionState::Fist {
                                host: *host,
                                bids,
                                action_taken,
                            },
                            target: *target,
                        }
                    }
                    _ => {
                        bail!("Invalid action.");
                    }
                }
            }
            (GameStage::AuctionInAction { state, target }, ActionInput::BidOptional(inner)) => {
                match state {
                    AuctionState::Circle {
                        starter,
                        current_player,
                        highest,
                    } => {
                        if *current_player == from.id {
                            match inner {
                                BidOptionalInner::Pass => {
                                    if *current_player == *starter {
                                        self.complete_transaction(*target, highest.1, highest.0)
                                    } else {
                                        let next = self.get_next_player_rounded(*current_player);
                                        GameStage::AuctionInAction {
                                            state: AuctionState::Circle {
                                                starter: *starter,
                                                current_player: next,
                                                highest: *highest,
                                            },
                                            target: *target,
                                        }
                                    }
                                }
                                BidOptionalInner::Bid(money) => {
                                    if money < highest.1 {
                                        bail!("The current price is higher than your offer.");
                                    }
                                    let highest = (from.id, money);
                                    let next = self.get_next_player_rounded(*current_player);
                                    GameStage::AuctionInAction {
                                        state: AuctionState::Circle {
                                            starter: *starter,
                                            current_player: next,
                                            highest,
                                        },
                                        target: *target,
                                    }
                                }
                            }
                        } else {
                            bail!("Not your turn yet.");
                        }
                    }
                    _ => {
                        bail!("Invalid action.");
                    }
                }
            }
            (GameStage::AuctionInAction { state, target }, ActionInput::MarkedReaction(inner)) => {
                match state {
                    AuctionState::Marked { current, price } => {
                        if *current == from.id {
                            if price.0 == *current {
                                self.complete_transaction(*target, price.1, from.id)
                            } else {
                                match inner {
                                    MarkedReactionInner::Accept => {
                                        self.test_enough_money(from.id, price.1)?;
                                        self.complete_transaction(*target, price.1, from.id)
                                    }
                                    MarkedReactionInner::Pass => {
                                        let next = self.get_next_player_rounded(*current);
                                        GameStage::AuctionInAction {
                                            state: AuctionState::Marked {
                                                current: next,
                                                price: *price,
                                            },
                                            target: *target,
                                        }
                                    }
                                }
                            }
                        } else {
                            bail!("Not your turn yet.");
                        }
                    }
                    _ => {
                        bail!("Invalid action.");
                    }
                }
            }
            (GameStage::AuctionInAction { state, target }, ActionInput::Call) => match state {
                AuctionState::Free {
                    host,
                    highest,
                    time_end,
                    calls,
                } => {
                    if *host == from.id {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64();
                        if now > *time_end {
                            if *calls == 2 {
                                self.complete_transaction(*target, highest.1, highest.0)
                            } else {
                                let calls = calls + 1;
                                let now = SystemTime::now();
                                let time_end = now + Duration::from_secs(3);
                                GameStage::AuctionInAction {
                                    state: AuctionState::Free {
                                        host: *host,
                                        highest: *highest,
                                        time_end: time_end
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs_f64(),
                                        calls,
                                    },
                                    target: *target,
                                }
                            }
                        } else {
                            bail!("Please wait for at least 3 seconds before calling.");
                        }
                    } else {
                        bail!("Not your turn yet.");
                    }
                }
                AuctionState::Fist {
                    host,
                    bids,
                    action_taken,
                } => {
                    if from.id != *host {
                        bail!("Invalid action.");
                    }
                    if action_taken.contains(&false) {
                        bail!("Somebody has not made their decision yet!");
                    }
                    let max = bids.iter().max().unwrap();
                    let max_index = bids.iter().position(|bid| *bid == *max).unwrap();
                    self.complete_transaction(*target, *max, max_index)
                }
                _ => {
                    bail!("Invalid action.");
                }
            },
            _ => {
                bail!("Invalid action.");
            }
        };
        self.stage = next_stage;

        Ok(())
    }

    fn gen_auction_state(&self, card: &Card, from: PlayerID, player_count: usize) -> AuctionState {
        match card.ty {
            AuctionType::Free => {
                let now = SystemTime::now();
                let time_end = now + Duration::from_secs(3);
                AuctionState::Free {
                    host: from,
                    highest: (from, 0 as Money),
                    time_end: time_end.duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
                    calls: 0,
                }
            }
            AuctionType::Circle => AuctionState::Circle {
                starter: from,
                current_player: self.get_next_player_rounded(from),
                highest: (from, 0 as Money),
            },
            AuctionType::Fist => AuctionState::Fist {
                host: from,
                bids: vec![0 as Money; player_count],
                action_taken: vec![false; player_count],
            },
            _ => unreachable!(),
        }
    }
}

fn play_card(deck: &mut Vec<Vec<Card>>, from: &mut Player, card_id: CardID) -> Result<Card> {
    let player_deck = deck.get_mut(from.id).unwrap();
    let index = player_deck
        .iter()
        .position(|card| card.id == card_id)
        .context("No such card.")?;
    let card = player_deck.remove(index);
    Ok(card)
}

