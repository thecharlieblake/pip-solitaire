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

use game::Game;

pub mod game;
pub mod utils;

///
/// Generates a game from a sorted deck
///
pub fn gen_game() -> Game {
    let deck = Default::default();
    Game::deal(deck)
}
