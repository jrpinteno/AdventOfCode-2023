use std::collections::BTreeMap;

use aoc_2023::utils;

#[derive(Clone, Debug)]
struct Hand {
   cards: Vec<u32>,
   joker_exists: bool,
   bid: u32
}

impl Hand {
   fn from(string: &String, joker_exists: bool) -> Self {
      let parts: Vec<_> = string.split_whitespace().into_iter().collect();
      let cards: Vec<_> = parts[0].to_string().chars().into_iter().map(|card| {
         Hand::parse_card(card, joker_exists)
      }).collect();

      Self {
         cards,
         joker_exists,
         bid: parts[1].parse().unwrap()
      }
   }

   fn parse_card(card: char, joker_exists: bool) -> u32 {
      match card {
         'T' => 10,
         'J' => if joker_exists { 1 } else { 11 },
         'Q' => 12,
         'K' => 13,
         'A' => 14,
         _ => card.to_digit(10).unwrap()
      }
   }

   fn create_play(&self) -> BTreeMap<u32, u32> {
      let mut play: BTreeMap<u32, u32> = BTreeMap::new();
   
      for card in &self.cards {
         *play.entry(*card).or_insert(0) += 1;
      }
   
      play
   }

   fn determine_play(&self) -> HandPlay {
      let play = self.create_play();
      let unique_cards_count = play.len();
      let mut sorted_values: Vec<_> = play.values().copied().collect();
      sorted_values.sort();
   
      match unique_cards_count {
         1 => HandPlay::FiveOfAKind,
         2 if sorted_values == [1, 4] => {
            if self.joker_exists && play.contains_key(&1) { HandPlay::FiveOfAKind } else { HandPlay::FourOfAKind }
         },
         2 if sorted_values == [2, 3] => {
            if self.joker_exists && play.contains_key(&1) { HandPlay::FiveOfAKind } else { HandPlay::FullHouse }
         },
         3 if sorted_values == [1, 1, 3] => {
            if self.joker_exists && play.contains_key(&1) { HandPlay::FourOfAKind } else { HandPlay::ThreeOfAKind }
         },
         3 if sorted_values == [1, 2, 2] => {
            if self.joker_exists && play.contains_key(&1) { 
               let count = *play.get(&1).unwrap();
               
               if count == 1 { HandPlay::FullHouse } else { HandPlay::FourOfAKind }
            } else { 
               HandPlay::TwoPair 
            }
         },
         4 => {
            if self.joker_exists && play.contains_key(&1) { HandPlay::ThreeOfAKind } else { HandPlay::OnePair }
         },
         _ => {
            if self.joker_exists && play.contains_key(&1) { HandPlay::OnePair } else { HandPlay::HighCard }
         }
      }
   }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandPlay {
   FiveOfAKind,
   FourOfAKind,
   FullHouse,
   ThreeOfAKind,
   TwoPair,
   OnePair,
   HighCard
}

fn get_winnings(hands: &mut Vec<Hand>) -> u32 {
   //hands.sort_by_key(|hand| (hand.determine_play(), &hand.cards));
   hands.sort_by(|lhs, rhs| {
      let hand_cmp = rhs.determine_play().cmp(&lhs.determine_play());
      if hand_cmp != std::cmp::Ordering::Equal {
         return hand_cmp
      }

      lhs.cards.cmp(&rhs.cards)
   });

   let mut winnings: u32 = 0;
   let mut i = 1;

   for hand in hands {
      winnings += hand.bid * i;
      i += 1;
   }

   winnings
}

fn part_1(input: &Vec<String>) -> u32 {
   let mut hands: Vec<_> = input
      .iter()
      .map(|play| { Hand::from(&play, false) })
      .collect();

   get_winnings(&mut hands)
}

fn part_2(input: &Vec<String>) -> u32 {
   let mut hands: Vec<_> = input
   .iter().map(|play| { Hand::from(&play, true) })
   .collect();

   get_winnings(&mut hands)
}


fn main() {
   let input = utils::read_file_to_vector("day7.txt").unwrap();
   let winnings_part_1 = part_1(&input);
   println!("{}", winnings_part_1);

   let winnings_part_2 = part_2(&input);
   println!("{}", winnings_part_2);
}

#[cfg(test)]
mod tests {
   use aoc_2023::utils::StringExtension;
	use super::*;

	const SAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

   #[test]
   fn test_sample_1() {
      let input = SAMPLE.to_string_vector();
      let mut hands: Vec<_> = input.iter().map(|play| { Hand::from(&play, false) }).collect();
      let winnings = get_winnings(&mut hands);

      assert_eq!(winnings, 6440);
   }

   #[test]
   fn test_sample_2() {
      let input = SAMPLE.to_string_vector();
      let mut hands: Vec<_> = input.iter().map(|play| { Hand::from(&play, true) }).collect();
      let winnings = get_winnings(&mut hands);

      assert_eq!(winnings, 5905);
   }

   #[test]
   fn test_tree_map() {
      let hand_1 = Hand::from(&"32T3K 765".to_string(), false);
      assert_eq!(hand_1.determine_play(), HandPlay::OnePair);

      let hand_2 = Hand::from(&"T55J5 684".to_string(), false);
      assert_eq!(hand_2.determine_play(), HandPlay::ThreeOfAKind);
   }
}