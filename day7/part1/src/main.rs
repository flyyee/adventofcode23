use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Entry {
    hand: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn string_to_cards(hand: &str) -> Vec<Card> {
    use Card::*;
    hand.as_bytes()
        .iter()
        .map(|&card| match card {
            b'A' => Ace,
            b'K' => King,
            b'Q' => Queen,
            b'J' => Jack,
            b'T' => Ten,
            b'9' => Nine,
            b'8' => Eight,
            b'7' => Seven,
            b'6' => Six,
            b'5' => Five,
            b'4' => Four,
            b'3' => Three,
            b'2' => Two,
            _ => panic!("cant match {}", card),
        })
        .collect::<Vec<Card>>()
}

fn identify_hand(hand: &str) -> HandType {
    let mut charmap = HashMap::<u8, u8>::new();
    for byte in hand.as_bytes() {
        charmap.entry(*byte).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut occurrences_desc = charmap.into_iter().map(|(_, y)| y).collect::<Vec<u8>>();
    occurrences_desc.sort_unstable_by(|a, b| Ord::cmp(b, a));
    occurrences_desc.push(0);
    let occurrences_desc: (u8, u8) = occurrences_desc
        .into_iter()
        .take(2)
        .collect_tuple()
        .unwrap();

    match occurrences_desc {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day7/part1/src/testcase.txt").unwrap();

    let mut entries = input
        .split('\n')
        .map(|x| {
            let (hand, bid) = x.split_once(' ').unwrap();
            Entry {
                hand: string_to_cards(hand).try_into().unwrap(),
                bid: bid.parse().unwrap(),
                hand_type: identify_hand(hand),
            }
        })
        .collect::<Vec<Entry>>();

    entries.sort_unstable_by(|lhs, rhs| {
        if lhs.hand_type != rhs.hand_type {
            lhs.hand_type.partial_cmp(&rhs.hand_type).unwrap()
        } else {
            for (x, y) in zip(lhs.hand.iter(), rhs.hand.iter()) {
                if x != y {
                    return x.partial_cmp(y).unwrap();
                }
            }
            cmp::Ordering::Equal
        }
    });

    let ans: usize = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| (i + 1) * entry.bid)
        .sum();

    println!("{ans}");
    // println!("{entries:?}");
}
