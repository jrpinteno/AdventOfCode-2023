use aoc_2023::utils;

fn union_elements_count(lhs: Vec<&str>, rhs: Vec<&str>) -> usize {
	lhs.iter()
		.filter(|&x| rhs.contains(x))
		.count()
}

fn get_card_score(card: &String) -> (u32, usize) {
	let numbers: Vec<_> = card.split(": ").skip(1).nth(0).unwrap().split(" | ").collect();
	let lottery: Vec<_> = numbers[0].split_whitespace().collect();
	let hold: Vec<_> = numbers[1].split_whitespace().collect();
	let matched_count = union_elements_count(lottery, hold);

	(if matched_count == 0 { 0 } else { u32::pow(2, (matched_count - 1) as u32) }, matched_count)
}

fn sum_cards_score(cards: &Vec<String>) -> (u32, u32) {
	let mut part1_sum = 0;
	let mut part2_sum = 0;

	let mut card_index = 0;
	let mut card_copies: Vec<u32> = vec![1; cards.len()];

	for card in cards {
		let (score, match_count) = get_card_score(card);

		part1_sum += score;
		part2_sum += card_copies[card_index];

		card_index += 1;

		for i in card_index..card_index+match_count {
			card_copies[i] += card_copies[card_index - 1];
		}
	}

	(part1_sum, part2_sum)
}

fn main() {
	let cards = utils::read_file_to_vector("input/2023/day4.txt").unwrap();
	let (part1_sum, part2_sum) = sum_cards_score(&cards);

	println!("{}", part1_sum);
	println!("{}", part2_sum)
}

#[cfg(test)]
mod tests {
	use super::*;
	use aoc_2023::utils::StringExtension;

	const SAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

	const SAMPLE_CARD: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

	#[test]
	fn test_matching_numbers() {
		let card = SAMPLE_CARD.to_string();
		let numbers: Vec<_> = card.split(": ").skip(1).nth(0).unwrap().split(" | ").collect();
		let lottery: Vec<_> = numbers[0].split_whitespace().collect();
		let hold: Vec<_> = numbers[1].split_whitespace().collect();
		let matched_count = union_elements_count(lottery, hold);

		assert_eq!(matched_count, 4);
	}

	#[test]
	fn test_single_card_score() {
		let score = get_card_score(&SAMPLE_CARD.to_string()).0;

		assert_eq!(score, 8)
	}

	#[test]
	fn test_sample_1() {
		let cards = SAMPLE.to_string_vector();
		let sum = sum_cards_score(&cards).0;

		assert_eq!(sum, 13);
	}

	#[test]
	fn test_sample_2() {
		let cards = SAMPLE.to_string_vector();
		let sum = sum_cards_score(&cards).1;

		assert_eq!(sum, 30);
	}
}
