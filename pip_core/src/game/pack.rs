use std::fmt;
use bimap::BiMap;
use std::slice::Iter;
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::str::FromStr;
use rand::{Isaac64Rng, SeedableRng, Rng};


#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Hash, Deserialize, Serialize)]
pub struct Deck (Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        let cards : Vec<Card> = (1..=13)
            .flat_map(|r| Suit::iterator()
                .map(move |s| Card{rank: Rank(r), suit: *s})
            ).collect();
        Deck(cards)
    }
}

impl Deck {
    pub fn shuffled(seed: u64) -> Self {
        let mut d = Self::default();
        Isaac64Rng::seed_from_u64(seed).shuffle(&mut d.0);
        d
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<Card> {
        self.0.iter()
    }
}

// Cards //


#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Hash)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let mut s_iter = s.chars();
        let rank_str = s_iter.next().ok_or(())?.to_string();
        let suit_str = s_iter.next().ok_or(())?.to_string();

        let rank = Rank::from_str(&rank_str)?;
        let suit = Suit::from_str(&suit_str)?;

        Ok(Card { rank, suit })
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct CardStrVisitor;

impl<'de> Visitor<'de> for CardStrVisitor {
    type Value = Card;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a rank")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error {

        match Self::Value::from_str(v) {
            Ok(r) => Ok(r),
            Err(()) => Err(de::Error::custom("failed to deserialize card")),
        }
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CardStrVisitor)
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match RANKMAP.get_by_right(&self) {
            Some(s) => write!(f, "{}", s),
            _ => panic!(),
        }
    }
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        match RANKMAP.get_by_left(&s) {
            Some(s) => Ok(*s),
            None => Err(()),
        }
    }
}

impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct RankStrVisitor;

impl<'de> Visitor<'de> for RankStrVisitor {
    type Value = Rank;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a rank")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error {

        match Self::Value::from_str(v) {
            Ok(r) => Ok(r),
            Err(()) => Err(de::Error::custom("failed to deserialize rank")),
        }
    }
}

impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RankStrVisitor)
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

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        use self::Suit::*;
        static SUITS: [Suit;  4] = [Clubs, Diamonds, Spades, Hearts];
        SUITS.into_iter()
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

impl FromStr for Suit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        match SUITMAP.get_by_left(&s) {
            Some(s) => Ok(*s),
            None => Err(()),
        }
    }
}

impl Serialize for Suit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct SuitStrVisitor;

impl<'de> Visitor<'de> for SuitStrVisitor {
    type Value = Suit;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a rank")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error {

        match Self::Value::from_str(v) {
            Ok(r) => Ok(r),
            Err(()) => Err(de::Error::custom("failed to deserialize suit")),
        }
    }
}

impl<'de> Deserialize<'de> for Suit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SuitStrVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

   #[test]
    fn default_is_sorted_deck() {
        let s : &str =
            "[AC, AD, AS, AH,
              2C, 2D, 2S, 2H,
              3C, 3D, 3S, 3H,
              4C, 4D, 4S, 4H,
              5C, 5D, 5S, 5H,
              6C, 6D, 6S, 6H,
              7C, 7D, 7S, 7H,
              8C, 8D, 8S, 8H,
              9C, 9D, 9S, 9H,
              TC, TD, TS, TH,
              JC, JD, JS, JH,
              QC, QD, QS, QH,
              KC, KD, KS, KH]";
        assert_eq!(Deck::default(), serde_yaml::from_str(s).unwrap());
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
            assert_eq!(val, serde_yaml::from_str(&str).unwrap());
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
            assert_eq!(val, serde_yaml::from_str(&str).unwrap());
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
            assert_eq!(val, serde_yaml::from_str(&str).unwrap());
            assert_eq!(str, val.to_string());
            assert_eq!(format!("---\n{}", &str), serde_yaml::to_string(&val).unwrap());
        }
    }
}