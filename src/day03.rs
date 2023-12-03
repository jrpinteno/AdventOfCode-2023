use aoc_2023::utils;
use aoc_2023::utils::Point;

#[derive(Debug, Copy, Clone)]
struct Part {
	number: u32,
	position: Point
}

impl Part {
	fn is_adjacent(&self, position: Point) -> bool {
		let length = self.number.ilog10();
		let horizontal = self.position.y.checked_sub(1).unwrap_or(0) ..= self.position.y + length + 1;
		let vertical = self.position.x.checked_sub(1).unwrap_or(0) ..= self.position.x + 1;

		horizontal.contains(&position.y) && vertical.contains(&position.x)
	}
}

fn is_valid_symbol(symbol: char, is_gear: bool) -> bool {
	(is_gear && symbol == '*') || symbol != '.' && symbol.is_ascii_punctuation()
}

fn extract_points(engine: &Vec<String>, is_gear: bool) -> Vec<Point> {
	let mut points = Vec::new();
	let mut i = 0;

	for row in engine {
		let mut j = 0;

		for c in row.chars() {
			if is_valid_symbol(c, is_gear) {
				points.push(Point { x: i, y: j });
			}

			j += 1;
		}

		i += 1;
	}

	points
}

fn get_numbers(input: &Vec<String>) -> Vec<Part> {
	input.iter().enumerate().flat_map(|(row_index, row)| {
		let mut column_index = 0;

		row.chars().enumerate().flat_map(move |(char_index, c)| {
			if char_index < column_index {
				return None
			}

			if let Some(digit) = c.to_digit(10) {
				let mut number = String::new();
				number.push(c);

				let mut next_char_index = char_index + 1;

				while let Some(next_char) = row.chars().nth(next_char_index) {
					if let Some(next_digit) = next_char.to_digit(10) {
						number.push(next_char);
						next_char_index += 1;
					} else {
						break;
					}
				}

				column_index = next_char_index;

				Some(Part {
					number: number.parse::<u32>().unwrap(),
					position: Point { x: row_index as u32, y: char_index as u32 }
				})
			} else {
				column_index += 1;
				None
			}
		})
	}).collect()
}

fn sum_parts(schema: &Vec<String>, is_gear: bool) -> u32 {
	let points = extract_points(&schema, is_gear);
	let parts = get_numbers(&schema);

	if !is_gear {
		parts.iter().filter(|&part| {
			points.iter().any(|&point| {
				part.is_adjacent(point)
			})
		}).map(|&part| part.number)
			.sum()
	} else {
		points.iter().filter_map(|&point| {
			let adjacent_parts: Vec<u32> = parts
				.iter()
				.filter(|part| part.is_adjacent(point))
				.map(|part| part.number)
				.collect();

			if adjacent_parts.len() == 2 {
				Some(adjacent_parts[0] * adjacent_parts[1])
			} else {
				None
			}
		}).sum()
	}
}

fn main() {
	let schema = utils::read_file_to_vector("input/2023/day3.txt").unwrap();
	let sum = sum_parts(&schema, false);
	println!("{}", sum);

	let sum_gears = sum_parts(&schema, true);
	println!("{}", sum_gears);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::StringExtension;
	use super::*;

	const SAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

	#[test]
	fn test_valid_symbols() {
		assert_eq!(is_valid_symbol('_', false), true);
		assert_eq!(is_valid_symbol('+', false), true);
		assert_eq!(is_valid_symbol('*', false), true);
		assert_eq!(is_valid_symbol('@', false), true);
		assert_eq!(is_valid_symbol('.', false), false);
		assert_eq!(is_valid_symbol('d', false), false);
		assert_eq!(is_valid_symbol('8', false), false);
	}

	#[test]
	fn test_extract_symbol_coordinate() {
		let sample: Vec<String> = SAMPLE.to_string_vector();

		let points = vec![
			Point { x: 1, y: 3 },
			Point { x: 3, y: 6 },
			Point { x: 4, y: 3 },
			Point { x: 5, y: 5 },
			Point { x: 8, y: 3 },
			Point { x: 8, y: 5 }
		];

		let extracted_points = extract_points(&sample, false);
		assert_eq!(extracted_points.len(), points.len());
		assert_eq!(points, extracted_points);
	}

	#[test]
	fn test_extract_numbers() {
		let sample: Vec<String> = SAMPLE.to_string_vector();

		let numbers = get_numbers(&sample);

		println!("{:?}", numbers);
	}

	#[test]
	fn test_adjacent() {
		let part = Part {
			number: 457,
			position: Point { x: 2, y: 5 }};

		assert_eq!(part.is_adjacent(Point { x: 2, y: 6}), true);
		assert_eq!(part.is_adjacent(Point { x: 1, y: 3}), false);
		assert_eq!(part.is_adjacent(Point { x: 1, y: 8}), true);
		assert_eq!(part.is_adjacent(Point { x: 2, y: 9}), false);
	}

	#[test]
	fn test_sample_1() {
		let sample: Vec<String> = SAMPLE.to_string_vector();
		let sum = sum_parts(&sample, false);

		assert_eq!(sum, 4361)
	}

	#[test]
	fn test_sample_2() {
		let sample: Vec<String> = SAMPLE.to_string_vector();
		let sum = sum_parts(&sample, true);

		assert_eq!(sum, 467835)
	}
}