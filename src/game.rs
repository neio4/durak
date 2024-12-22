use std::{cmp::Ordering, fmt};
use std::marker::ConstParamTy;

/// ```not_rust
///         Trump Suit
///         |=|
/// 0 0 0 0 0 0 0 0
/// |=====|     |=|
///  Rank       Suit
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub packed: u8,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Deck<const SIZE: usize, const TRUMP: Suit> {
    pub cards: [u8; SIZE]
}

// impl<T> Default for Deck<52, T> {
//     fn default() -> Self {
//             for s in 0..=3u8 {
//                 let suit: Suit = unsafe { std::mem::transmute(s) };
//                 for rnk in 2..=14u8 {
//                     let rank: Rank = unsafe { std::mem::transmute(rnk) };
//                 }
//             }

//     }
// }



impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.get_rank(), self.get_suit())
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("packed", &self.packed)
            .field("binary", &format!("{:#08b}", self.packed))
            .field("trump", &format!("{:?}", self.get_trump_suit()))
            .field("suit", &format!("{:?}", self.get_suit()))
            .field("rank", &format!("{:?}", self.get_rank()))
            .finish()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let same_suit = self.same_suit(other);
        let self_is_trump = self.is_trump();
        let other_is_trump = other.is_trump();

        match (same_suit, self_is_trump, other_is_trump) {
            (true, _, _) => Some(self.cmp_rank(other)),
            (false, true, _) => Some(Ordering::Greater),
            (false, false, true) => Some(Ordering::Less),
            (false, false, false) => None,
        }
    }
}

impl Ord for Card {
    /// assumes that the comparison is valid (same suit or one card is trump)
    fn cmp(&self, other: &Self) -> Ordering {
        let same_suit = self.same_suit(other);
        let self_is_trump = self.is_trump();
        let other_is_trump = other.is_trump();

        match (same_suit, self_is_trump, other_is_trump) {
            (true, _, _) => self.cmp_rank(other),
            (false, true, _) => Ordering::Greater,
            (false, false, true) => Ordering::Less,
            (false, false, false) => unreachable!("bad cmp"),
        }
    }
}


impl Card {
    const SUIT_MASK: u8 = 0b00000011;
    const TRUMP_SUIT_MASK: u8 = 0b00001100;
    const RANK_MASK: u8 = 0b11110000;

    #[inline]
    pub const fn is_trump(&self) -> bool {
        let trump_suit = (self.packed << 4) & 0b11000000;
        let suit = self.packed << 6;
        trump_suit == suit
    }

    #[inline]
    pub const fn same_suit(&self, card2: &Self) -> bool {
        let suit1 = self.packed << 6;
        let suit2 = card2.packed << 6;
        suit1 == suit2
    }

    #[inline]
    pub const fn is_bigger(&self, card2: &Self) -> bool {
        self.packed >> 4 > card2.packed >> 4
    }


    #[inline]
    pub fn cmp_rank(&self, card2: &Self) -> Ordering {
        (self.packed & Self::RANK_MASK).cmp(&(card2.packed & Self::RANK_MASK))
    }


    pub const fn new(suit: Suit, trump_suit: Suit, rank: Rank) -> Self {
        let suit_bits = suit as u8;
        let trump_suit_bits = trump_suit as u8;
        let rank_bits = rank as u8;

        let packed = (rank_bits << 4)
            | ((trump_suit_bits << 2) & Self::TRUMP_SUIT_MASK)
            | (suit_bits & Self::SUIT_MASK);

        Card { packed }
    }
    pub const fn get_trump_suit(&self) -> Suit {
        let trump_suit = (self.packed & Self::TRUMP_SUIT_MASK) >> 2;
        unsafe { std::mem::transmute(trump_suit) }
    }

    pub const fn get_suit(&self) -> Suit {
        let suit = self.packed & Self::SUIT_MASK;
        unsafe { std::mem::transmute(suit) }
    }

    pub const fn get_rank(&self) -> Rank {
        let rank = (self.packed & Self::RANK_MASK) >> 4;
        unsafe { std::mem::transmute(rank) }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, ConstParamTy)]
pub enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparisons() {
        for s in 0..=3u8 {
            let suit1: Suit = unsafe { std::mem::transmute(s) };
            for s2 in 0..=3u8 {
                let suit2: Suit = unsafe { std::mem::transmute(s2) };
                for rnk in 2..=14u8 {
                    let rank1: Rank = unsafe { std::mem::transmute(rnk) };
                    for rnk2 in 2..=14u8 {
                        let rank2: Rank = unsafe { std::mem::transmute(rnk2) };
                        for t in 0..=3u8 {
                            let trump_suit: Suit = unsafe { std::mem::transmute(t) };
                            let card1 = Card::new(suit1, trump_suit, rank1);
                            let card2 = Card::new(suit2, trump_suit, rank2);
                            println!("{suit1:?} {rank1:?}");
                            println!("{suit2:?} {rank2:?}");
                            assert_eq!(rank1 > rank2, card1.is_bigger(&card2));
                            assert_eq!(rank1 < rank2, card2.is_bigger(&card1));
                            assert_eq!(suit1 == trump_suit, card1.is_trump());
                            assert_eq!(suit2 == trump_suit, card2.is_trump());
                            assert_eq!(suit1 == suit2, card1.same_suit(&card2));
                            assert_eq!(card2.same_suit(&card1), card1.same_suit(&card2));
                            println!("{card1:?}");
                            println!("{card2:?}");
                            assert_eq!(suit1, card1.get_suit());
                            assert_eq!(suit2, card2.get_suit());
                            assert_eq!(trump_suit, card1.get_trump_suit());
                            assert_eq!(trump_suit, card2.get_trump_suit());

                            if suit1 == suit2 {
                                assert_eq!(rank1 > rank2, card1 > card2);
                                assert_eq!(rank2 > rank1, card2 > card1);
                                assert_eq!(rank1 < rank2, card1 < card2);
                                assert_eq!(rank2 < rank1, card2 < card1);
                            } else if suit1 == trump_suit {
                                if suit2 == trump_suit {
                                    assert_eq!(rank1 > rank2, card1 > card2);
                                    assert_eq!(rank2 > rank1, card2 > card1);
                                    assert_eq!(rank1 < rank2, card1 < card2);
                                    assert_eq!(rank2 < rank1, card2 < card1);
                                } else {
                                    println!(" {card1:?} > {card2:?} ");
                                    assert!(card1 > card2);
                                    println!(" {card2:?} < {card1:?} ");
                                    assert!(card2 < card1);
                                }
                            } else if suit2 == trump_suit {
                                if suit1 == trump_suit {
                                    assert_eq!(rank1 > rank2, card1 > card2);
                                    assert_eq!(rank2 > rank1, card2 > card1);
                                    assert_eq!(rank1 < rank2, card1 < card2);
                                    assert_eq!(rank2 < rank1, card2 < card1);
                                } else {
                                    println!(" {card1:?} < {card2:?} ");
                                    assert!(card1 < card2);
                                    println!(" {card2:?} > {card1:?} ");
                                    assert!(card2 > card1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
