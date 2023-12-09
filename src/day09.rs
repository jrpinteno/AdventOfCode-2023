use aoc_2023::utils;

fn get_next_number_sum(vector: &Vec<i32>) -> i32 {
	let mut result = vector.clone();
	let mut last_values: Vec<i32> = Vec::new();
	last_values.push(*result.last().unwrap());

	while result.iter().any(|&x| x != 0) {
		result = result.windows(2)
			.map(|pair| pair[1] - pair[0])
			.collect();

		last_values.push(*result.last().unwrap());
	}

	last_values.iter().fold(0, |acc, x| acc + x )
}

fn get_previous_number_sum(vector: &Vec<i32>) -> i32 {
	let mut result = vector.clone();
	let mut last_values: Vec<i32> = Vec::new();
	last_values.push(*result.first().unwrap());

	while result.iter().any(|&x| x != 0) {
		result = result.windows(2)
			.map(|pair| pair[1] - pair[0])
			.collect();

		last_values.push(*result.first().unwrap());
	}

	last_values.iter().rev().skip(1).fold(0, |acc, &x| x - acc)
}

fn main() {
	let input = utils::read_file_to_line_number_vector("input/2023/day9.txt").unwrap();

	let result = input.iter().fold((0, 0), |(acc1, acc2), temp_vector| {
		(
			acc1 + get_next_number_sum(&temp_vector),
			acc2 + get_previous_number_sum(&temp_vector)
		)
	});

	println!("Part 1: {}", result.0);
	println!("Part 2: {}", result.1);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::StringExtension;
	use super::*;

	const SAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

	#[test]
	fn test_sample_1() {
		let temp: Vec<_> = SAMPLE.to_string_vector().iter().map(|line| {
				line.split_whitespace().filter_map(|num| num.parse::<i32>().ok()).collect::<Vec<_>>()
			}).collect();

		let result = temp.iter().fold(0, |acc, temp_vector| {
			acc + get_next_number_sum(&temp_vector)
		});

		assert_eq!(result, 114);
	}

	#[test]
	fn test_sample_2() {
		let temp: Vec<_> = SAMPLE.to_string_vector().iter().map(|line| {
			line.split_whitespace().filter_map(|num| num.parse::<i32>().ok()).collect::<Vec<_>>()
		}).collect();

		let result = temp.iter().fold(0, |acc, temp_vector| {
			acc + get_previous_number_sum(&temp_vector)
		});

		assert_eq!(result, 2);
	}

	#[test]
	fn test_get_difference() {
		let vector = vec![0, 3, 6, 9, 12, 15];

		let result: Vec<_> = vector
			.windows(2)
			.map(|pair| pair[1] - pair[0])
			.collect();

		assert_eq!(result, [3, 3, 3, 3, 3]);
	}

	#[test]
	fn test_next_number_sum() {
		let vector = vec![0, 3, 6, 9, 12, 15];
		let next_number_sum = get_next_number_sum(&vector);

		assert_eq!(next_number_sum, 18);
	}

	#[test]
	fn test_backwards() {
		let vector = vec![10, 13, 16, 21, 30, 45];
		let result = get_previous_number_sum(&vector);

		assert_eq!(result, 5);
	}
}