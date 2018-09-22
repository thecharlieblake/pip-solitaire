//!
//! # The core solitaire library for pip
//!
//! Contains only features relevant to solitaire, *not* search.
//!
#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;
extern crate bimap;

mod game;

use game::Game;
use game::pack::Deck;

///
/// Generates a game from a sorted deck
///
pub fn gen_game() -> Game {
    let deck = Deck::sorted();
    Game::deal(deck)
}
