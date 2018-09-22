use std::fmt;
use std::error::Error;
use std::str::FromStr;
use bimap::BiMap;
use std::slice::Iter;


#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn sorted() -> Self {
        let cards : Vec<Card> =
            Suit::iterator()
                .flat_map(|s| (1..=13)
                    .map(move |r| Card{rank: Rank(r), suit: *s})
                )
                .collect();
        Self {cards}
    }
}

// Cards //

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Hash)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.chars();
        let rank_str = s_iter.next().ok_or("missing card rank")?.to_string();
        let suit_str = s_iter.next().ok_or("missing card suit")?.to_string();
        let rank = Rank::from_str(&rank_str)?;
        let suit = Suit::from_str(&suit_str)?;
        Ok(Card { rank, suit })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

// Ranks //

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Rank (
    u8,
);

lazy_static! {
    static ref RANKMAP: BiMap<&'static str, Rank> = {
        let mut m = BiMap::new();
        m.insert("A", Rank(1));
        m.insert("2", Rank(2));
        m.insert("3", Rank(3));
        m.insert("4", Rank(4));
        m.insert("5", Rank(5));
        m.insert("6", Rank(6));
        m.insert("7", Rank(7));
        m.insert("8", Rank(8));
        m.insert("9", Rank(9));
        m.insert("T", Rank(10));
        m.insert("J", Rank(11));
        m.insert("Q", Rank(12));
        m.insert("K", Rank(13));
        m
    };
}

quick_error! {
    #[derive(Debug)]
    pub enum ParseRankError {
        InvalidRank {
            description("rank is not valid")
        }
    }
}

impl FromStr for Rank {
    type Err = ParseRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        match RANKMAP.get_by_left(&s) {
            Some(r) => Ok(*r),
            None => Err(ParseRankError::InvalidRank),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match RANKMAP.get_by_right(&self) {
            Some(s) => write!(f, "{}", s),
            None => panic!("Invalid rank: {}", self.0),
        }
    }
}

// Suits //

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

lazy_static! {
    static ref SUITMAP: BiMap<&'static str, Suit> = {
        let mut m = BiMap::new();
        m.insert("S", Suit::Spades);
        m.insert("H", Suit::Hearts);
        m.insert("C", Suit::Clubs);
        m.insert("D", Suit::Diamonds);
        m
    };
}


quick_error! {
    #[derive(Debug)]
    pub enum ParseSuitError {
        InvalidSuit {
            description("Suit is invalid")
        }
    }
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        use self::Suit::*;
        static SUITS: [Suit;  4] = [Clubs, Diamonds, Spades, Hearts];
        SUITS.into_iter()
    }
}

impl FromStr for Suit {
    type Err = ParseSuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        match SUITMAP.get_by_left(&s) {
            Some(s) => Ok(*s),
            None => Err(ParseSuitError::InvalidSuit),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match SUITMAP.get_by_right(&self) {
            Some(s) => write!(f, "{}", s),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_deck() {
        assert_eq!(Deck { cards: vec![
            "AC", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C", "TC", "JC", "QC", "KC",
            "AD", "2D", "3D", "4D", "5D", "6D", "7D", "8D", "9D", "TD", "JD", "QD", "KD",
            "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "TS", "JS", "QS", "KS",
            "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "TH", "JH", "QH", "KH",
        ].iter().map(|c| c.parse().unwrap()).collect()},
        Deck::sorted());
    }

    #[test]
    fn card() {
        use self::Suit::*;

        for c in [
            ("AS", Card { rank: Rank(1), suit: Spades }),
            ("6C", Card { rank: Rank(6), suit: Clubs }),
            ("TH", Card { rank: Rank(10), suit: Hearts }),
            ("KD", Card { rank: Rank(13), suit: Diamonds }),
        ].iter() {
            let str = c.0;
            let val = c.1;

            assert_eq!(val, str.parse().unwrap());
            assert_eq!(str, val.to_string());
        }
    }

    #[test]
    fn rank() {
        for r in [
            ("A", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("T", 10),
            ("J", 11),
            ("Q", 12),
            ("K", 13),
        ].iter() {
            let str = r.0;
            let val = Rank(r.1);

            assert_eq!(val, str.parse().unwrap());
            assert_eq!(str, val.to_string());
        }
    }

    #[test]
    fn suit() {
        use self::Suit::*;
        for s in [
            ("S", Spades),
            ("H", Hearts),
            ("C", Clubs),
            ("D", Diamonds),
        ].iter() {
            let str = s.0;
            let val = s.1;

            assert_eq!(val, str.parse().unwrap());
            assert_eq!(str, val.to_string());
        }
    }
}