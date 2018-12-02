//!
//! # The core solitaire library for pip
//!
//! Contains only features relevant to solitaire, *not* search.
//!
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;
extern crate bimap;
extern crate itertools;
extern crate regex;
extern crate rand;

use game::Game;
use game::pack::{Deck, Rank};

pub mod game;
pub mod utils;


pub struct Options {
    pub seed: u64,
    pub max_rank: Rank,
}

pub fn gen_game(ops: Options) -> Game {
    let deck = Deck::shuffled(ops.seed, ops.max_rank);
    Game::deal(deck)
}

pub fn gen_default_game() -> Game {
    let deck = Deck::default();
    Game::deal(deck)
}
