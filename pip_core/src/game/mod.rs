use std::fmt::{ Display, Formatter, Error };

use self::pack::{ Deck, Card };

pub mod pack;

pub struct Game {
    foundations: Vec<Pile>,
    tableau_piles: Vec<Pile>,
}

impl Game {
    pub fn deal(_deck: Deck) -> Game {
        unimplemented!()
    }
}

impl Display for Game {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

pub struct Pile {
    cards: Vec<Card>
}
