include!(concat!(env!("OUT_DIR"), "/card_list.rs"));

use rand::prelude::*;

pub fn pick_into(pool: &mut Vec<Card>, count: usize, target: &mut Vec<Card>) {
    for _ in 0..count {
        let i = thread_rng().gen_range(0..pool.len());
        target.push(pool.remove(i));
    }
}

pub fn pick(pool: &mut Vec<Card>, count: usize) -> Vec<Card> {
    let mut result = Vec::with_capacity(count);
    pick_into(pool, count, &mut result);
    result
}

