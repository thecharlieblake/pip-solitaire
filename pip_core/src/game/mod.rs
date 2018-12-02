use self::pack::{ Deck, Card };

pub mod pack;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Game {
    foundations: Vec<Pile>,
    tableau_piles: Vec<Pile>,
}

impl Game {
    fn default_foundations_count() -> usize {
        4
    }

    fn default_tableau_count() -> usize {
        13
    }

    fn default_foundations() -> Vec<Pile> {
        (0..Self::default_foundations_count()).map(|_| Default::default()).collect()
    }

    fn default_tableau_piles() -> Vec<Pile> {
        (0..Self::default_tableau_count()).map(|_| Default::default()).collect()
    }

    pub fn deal(mut deck: Deck) -> Self {
        let foundations = Self::default_foundations();
        let mut tableau_piles = Self::default_tableau_piles();

        let mut deck_it = deck.iter_mut();
        'l: loop {
            for pile in tableau_piles.iter_mut() {
                match deck_it.next() {
                    Some(card) => pile.place(*card),
                    None => break 'l,
                }
            }
        }

        Self {
            foundations,
            tableau_piles,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            foundations: Self::default_foundations(),
            tableau_piles: Self::default_tableau_piles(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, Serialize, Deserialize)]
pub struct Pile (Vec<Card>);

impl Pile {
    fn place(&mut self, card: Card) {
        self.0.push(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils;
    use serde_yaml;

    #[test]
    fn default_game() {
        let game = Game::deal(Deck::default());
        let yaml_str = utils::yaml::to_pretty_string(&game);

        assert_eq!(game, serde_yaml::from_str(&yaml_str).unwrap());
    }
}