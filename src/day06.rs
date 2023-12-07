use aoc_2023::utils;

#[derive(Debug)]
struct Race {
	time: u64,
	distance: u64
}

impl Race {
	fn can_complete_race(&self, acceleration_time: u64) -> bool {
		let raced_distance = acceleration_time * (self.time - acceleration_time);
		self.distance < raced_distance
	}

	fn time_acceleration_range(&self) -> (u64, u64) {
		let a = 1.0;
		let b: f64 = 0.0 - self.time as f64;

		let determinant: f64 = (f64::powf(b, 2.0) - 4.0 * a * self.distance as f64).sqrt();
		let minus_b = 0.0 - b;
		let divisor = 2.0 * a;

		let t_1 = (minus_b + determinant) / divisor;
		let t_2 = (minus_b - determinant) / divisor;

		(t_2.ceil() as u64, t_1.floor() as u64)
	}
}

fn get_races(input: &Vec<Vec<u64>>) -> Vec<Race> {
	input
		.get(0).into_iter().flatten()
		.zip(input
			.get(1)
			.into_iter()
			.flatten())
		.map(|(&time, &distance)| Race { time, distance })
		.collect()
}

fn part1(input: &Vec<String>) -> usize {
	let parsed: Vec<Vec<u64>> = input.iter()
		.map(|line| {
			line.split_whitespace()
				.skip(1)
				.filter_map(|val| val.parse().ok())
				.collect()
		})
		.collect();

	let races: Vec<_> = get_races(&parsed);

	let finished_races: Vec<_> = races.iter().map(|race| {
		(0 .. race.time).into_iter().filter(|&acceleration_time| {
			race.can_complete_race(acceleration_time)
		}).count()
	}).collect();

	let result = finished_races.iter().fold(1, |acc, count| acc * count);
	result
}

fn parse_correct_kerning(input: &Vec<String>) -> Vec<u64> {
	input
		.iter()
		.map(|line| {
			line.split(": ")
				.skip(1)
				.next()
				.unwrap()
				.split_whitespace()
				.collect::<Vec<_>>()
				.join("")
				.parse::<u64>()
				.unwrap()
		}).collect()
}

fn part2(input: &Vec<String>) -> u64 {
	let race: Vec<_> = parse_correct_kerning(input);

	let result = vec![Race { time: race[0], distance: race[1] }].iter().fold(1, |acc, race| {
		let range = race.time_acceleration_range();
		acc * (range.1 - range.0 + 1)
	});

	result
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day6.txt").unwrap();
	let part1 = part1(&input);
	println!("Part 1: {}", part1);

	let part2 = part2(&input);
	println!("Part 2: {}", part2);

}

#[cfg(test)]
mod tests {
	use super::*;

	const SAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200";

	#[test]
	fn test_sample_1() {
		let parsed: Vec<_> = SAMPLE
			.lines()
			.filter(|line| !line.trim().is_empty())
			.map(|line| { line.split_whitespace()
				.skip(1)
				.filter_map(|val| val.parse().ok())
				.collect() })
			.collect();

		let races: Vec<_> = get_races(&parsed);

		let finished_races: Vec<_> = races.iter().map(|race| {
			(1 .. race.time).into_iter().filter(|&acceleration_time| {
				race.can_complete_race(acceleration_time)
			}).count()
		}).collect();

		let result = finished_races.iter().fold(1, |acc, count| acc * count);

		assert_eq!(result, 288);
	}

	#[test]
	fn test_sample_2() {
		let input: Vec<_> = SAMPLE
			.lines()
			.map(|line| { line.to_string() })
			.collect();

		let race = parse_correct_kerning(&input);

		let result = vec![Race { time: race[0], distance: race[1] }].iter().fold(1, |acc, race| {
			let range = race.time_acceleration_range();
			acc * (range.1 - range.0 + 1)
		});

		assert_eq!(result, 71503);
	}

	#[test]
	fn test_complete_race() {
		let race1 = Race { time: 7, distance: 9 };
		let race2 = Race { time: 15, distance: 40 };

		for acceleration_time in 0 ..= race1.time {
			let result1 = race1.can_complete_race(acceleration_time);
			println!("Accel {}: {}", acceleration_time, result1);
		}

		for acceleration_time in 0 ..= race2.time {
			let result1 = race2.can_complete_race(acceleration_time);
			println!("Accel {}: {}", acceleration_time, result1);
		}
	}
}