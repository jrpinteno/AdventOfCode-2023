use std::fs::File;
use std::io::{self, BufRead};

fn read_file_to_vector(file_path: &str) -> Result<Vec<String>, io::Error> {
	let file = File::open(file_path)?;
	let reader = io::BufReader::new(file);

	let lines_vec: Vec<String> = reader
		.lines()
		.map(|line| line.unwrap().trim().to_string())
		.filter(|line| !line.is_empty())
		.collect();

	Ok(lines_vec)
}

fn play_allowed(color: &str, quantity: &str, max_red: u32, max_green: u32, max_blue: u32) -> bool {
	match color {
		"red" => quantity.trim().parse::<u32>().unwrap_or(0) <= max_red,
		"green" => quantity.trim().parse::<u32>().unwrap_or(0) <= max_green,
		"blue" => quantity.trim().parse::<u32>().unwrap_or(0) <= max_blue,
		_ => false,
	}
}

fn sum_index(games: Vec<String>) -> u32 {
	games
		.iter()
		.map(|game| {
			game.split(": ").find_map(|word| {
				if let Some(number) = word.strip_prefix("Game ") {
					number.parse::<u32>().ok()
				} else {
					None
				}
			})
		})
		.filter_map(|number| number)
		.fold(0, |acc, num| acc + num)
}

fn filter_games(games: &Vec<String>) -> Vec<String> {
	games
		.iter()
		.filter(|&game| {
			game.split(": ").skip(1).all(|plays| {
				plays.split("; ").all(|play| {
					play.split(", ").collect::<Vec<_>>().iter().all(|colors| {
						let hand: Vec<_> = colors.split(" ").collect();

						if let (Some(quantity), Some(color)) = (hand.get(0), hand.get(1)) {
							return play_allowed(color, quantity, 12, 13, 14);
						} else {
							false
						}
					})
				})
			})
		})
		.cloned()
		.collect()
}

fn get_min_pieces(games: &Vec<String>) -> Vec<(u32, u32, u32)> {
	let game_counts: Vec<(u32, u32, u32)> = games
		.iter()
		.map(|game| {
			game.split(": ").nth(1).map_or((0, 0, 0), |part| {
				part.split("; ")
					.flat_map(|section| section.split(", "))
					.fold(
						(0, 0, 0),
						|(mut red_count, mut green_count, mut blue_count), count_str| {
							let parts: Vec<_> = count_str.split_whitespace().collect();

							if parts.len() == 2 {
								let count = parts[0].parse::<u32>().unwrap_or(0);

								match parts[1] {
									"red" => red_count = red_count.max(count),
									"green" => green_count = green_count.max(count),
									"blue" => blue_count = blue_count.max(count),
									_ => (),
								}
							}

							(red_count, green_count, blue_count)
						},
					)
			})
		})
		.collect();

	game_counts
}

fn main() {
	let games = read_file_to_vector("input/2023/day2.txt").unwrap();
	let sum = sum_index(filter_games(&games));
	println!("{}", sum);

	let min_pieces = get_min_pieces(&games);
	let power_sum = min_pieces.iter().fold(0, |sum, &(x, y, z)| sum + x * y * z);
	println!("{}", power_sum);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_color_allowed() {
		let color = "red";
		let quantity = "3";

		assert_eq!(play_allowed(color, quantity, 3, 5, 6), true);
	}

	#[test]
	fn test_color_not_allowed() {
		let color = "red";
		let quantity = "3";

		assert_eq!(play_allowed(color, quantity, 2, 5, 6), false);
	}

	#[test]
	fn test_sample_1() {
		let games = [
			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		]
		.iter()
		.map(|&s| s.to_string())
		.collect();

		let filtered_games = filter_games(&games);
		assert_eq!(8, sum_index(filtered_games));
	}

	#[test]
	fn test_sample_2() {
		let games = [
			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
			"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
			"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
		]
		.iter()
		.map(|&s| s.to_string())
		.collect();

		let min_pieces = get_min_pieces(&games);
		let power_sum = min_pieces.iter().fold(0, |sum, &(x, y, z)| sum + x * y * z);

		assert_eq!(2286, power_sum);
	}
}
