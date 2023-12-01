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

	println!("{}", sum);

	return sum;
}

fn main() {
	let input1 = read_lines("input/2023/day1.txt");
	part1(input1);
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
}