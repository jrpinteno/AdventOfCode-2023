use aoc_2023::utils;

fn union_elements_count(lhs: Vec<&str>, rhs: Vec<&str>) -> usize {
	lhs.iter()
		.filter(|&x| rhs.contains(x))
		.count()
}

fn get_card_score(card: &String) -> u32 {
	let numbers: Vec<_> = card.split(": ").skip(1).nth(0).unwrap().split(" | ").collect();
	let lottery: Vec<_> = numbers[0].split_whitespace().collect();
	let hold: Vec<_> = numbers[1].split_whitespace().collect();
	let matched_count = union_elements_count(lottery, hold);

	if matched_count == 0 { 0 } else { u32::pow(2, (matched_count - 1) as u32) }
}

fn sum_cards_score(cards: &Vec<String>) -> u32 {
	let mut sum = 0;

	for card in cards {
		sum += get_card_score(card);
	}

	sum
}

fn main() {
	let cards = utils::read_file_to_vector("input/2023/day4.txt").unwrap();
	let sum = sum_cards_score(&cards);

	println!("{}", sum);
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
	fn test_single_card_score() {
		let score = get_card_score(&SAMPLE_CARD.to_string());

		assert_eq!(score, 8)
	}

	#[test]
	fn test_sample_1() {
		let cards = SAMPLE.to_string_vector();
		let sum = sum_cards_score(&cards);

		assert_eq!(sum, 13);
	}

	#[test]
	fn test_sample_2() {
	}
}
