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
use game::pack::Deck;

pub mod game;
pub mod utils;

///
/// Generates a game from a sorted deck
///
pub fn gen_game(seed: u64) -> Game {
    let deck = Deck::shuffled(seed);
    Game::deal(deck)
}

pub fn gen_default_game() -> Game {
    let deck = Deck::default();
    Game::deal(deck)
}
