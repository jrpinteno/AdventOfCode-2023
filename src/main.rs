use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
	let mut result = Vec::new();

	for line in read_to_string(filename).unwrap().lines() {
		result.push(line.to_string())
	}

	result
}

fn part1(input: Vec<String>) -> u32 {
	let mut sum = 0;

	for line in input {
		let mut first_digit= '\0';
		let mut second_digit= '\0';

		for character in line.chars() {
			if character.is_numeric() {
				first_digit = character;
				break;
			}
		}

		for character in line.chars().rev() {
			if character.is_numeric() {
				second_digit = character;
				break;
			}
		}

		if first_digit == '\0' || second_digit == '\0' {
			continue;
		}

		let number = first_digit.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap();
		sum += number;
	}

	sum
}

fn part2(input: Vec<String>) -> u32 {
	let numbers = [
		"zero",
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
		"zero",
	];

	let mut sum = 0;

	for line in input {
		let mut digits = Vec::<u32>::new();
		let mut i = 0;

		for c in line.chars() {
			if c.is_numeric() {
				digits.push(c.to_digit(10).unwrap());
			} else {
				for (index, number) in numbers.iter().enumerate() {
					let substring = &line[i ..];

					if substring.starts_with(number) {
						digits.push(index as u32);
						break;
					}
				}
			}

			i += 1;
		}

		let first_digit = digits.first().unwrap();
		let second_digit = digits.last().unwrap();

		sum += first_digit * 10 + second_digit;
	}

	sum
}

fn main() {
	let input1 = read_lines("input/2023/day1.txt");
	println!("{}", part1(input1));

	let input2 = read_lines("input/2023/day1.txt");
	println!("{}", part2(input2));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() {
		let sample = "
			1abc2
			pqr3stu8vwx
			a1b2c3d4e5f
			treb7uchet
		";

		let mut lines_vec = Vec::new();

		for line in sample.lines() {
			lines_vec.push(line.to_string());
		}

		assert_eq!(142, part1(lines_vec));
	}

	#[test]
	fn test_parse2() {
		let sample = "
			two1nine
			eightwothree
			abcone2threexyz
			xtwone3four
			4nineeightseven2
			zoneight234
			7pqrstsixteen";

		let lines_vec: Vec<String> = sample
			.lines()
			.map(|line| line.trim().to_string())
			.filter(|line| !line.is_empty())
			.collect();

		assert_eq!(281, part2(lines_vec));
	}
}