extern crate pip_core;

use pip_core::*;

#[test]
fn gen_game_correct_order() {
    assert_eq!(
"---
foundations:
  - []
  - []
  - []
  - []
tableau_piles:
  - [AC, AD, AS, AH]
  - [2C, 2D, 2S, 2H]
  - [3C, 3D, 3S, 3H]
  - [4C, 4D, 4S, 4H]
  - [5C, 5D, 5S, 5H]
  - [6C, 6D, 6S, 6H]
  - [7C, 7D, 7S, 7H]
  - [8C, 8D, 8S, 8H]
  - [9C, 9D, 9S, 9H]
  - [TC, TD, TS, TH]
  - [JC, JD, JS, JH]
  - [QC, QD, QS, QH]
  - [KC, KD, KS, KH]
",
        utils::yaml::to_pretty_string(&gen_default_game())
    )
}
