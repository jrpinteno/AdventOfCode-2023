use std::collections::HashMap;

use aoc_2023::utils::utils;
use aoc_2023::utils::{point::*, grid::*};


fn get_first_direction(start: Point, grid: &Grid) -> Option<Direction> {
	let map = HashMap::from([
		(Direction::North, "|LJ"),
		(Direction::South, "|FJ"),
		(Direction::East, "-LF"),
		(Direction::West, "-7J"),
	]);

	for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
		let new_point = start + direction.point();
		if let Some(value) = grid.get_value(new_point.x as usize, new_point.y as usize) {
			let is_valid = map.get(&direction).unwrap().contains(value);

			if is_valid {
				return Some(direction)
			}
		}
	}

	None
}

fn traverse_pipes(start: Point, grid: &Grid) -> i32 {
	let mut current_location = start.clone();

	let mut direction = get_first_direction(start, &grid).unwrap();
	current_location.move_to(&direction);

	let mut distance = 1;

	while start != current_location {
		direction = match grid.get_value(current_location.x as usize, current_location.y as usize).unwrap() {
			'L' => if direction == Direction::South { Direction::East } else { Direction::North },
			'J' => if direction == Direction::East { Direction::North } else { Direction::West },
			'7' => if direction == Direction::East { Direction::South } else { Direction::West },
			'F' => if direction == Direction::North { Direction::East } else { Direction::South },
			'|' | '-' => direction,
			_ => panic!("Oops")
		};

		current_location.move_to(&direction);
		distance += 1;
	}

	distance / 2
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day10.txt").unwrap();
	let grid = Grid::from_string_vec(&input);
	let start = Point::from(grid.find('S').unwrap());
	let distance = traverse_pipes(start, &grid);

	println!("Distance: {}", distance);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::utils::StringExtension;

use super::*;

	const SAMPLE_1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

	const SAMPLE_2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

	#[test]
	fn test_sample_1() {
		let input = SAMPLE_1.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let distance = traverse_pipes(start, &grid);

		assert_eq!(distance, 4);
	}

	#[test]
	fn test_sample_2() {
		let input = SAMPLE_2.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let distance = traverse_pipes(start, &grid);

		assert_eq!(distance, 8);
	}

	#[test]
	fn test_sample_3() {

	}

	#[test]
	fn test_print_grid() {
		let input = SAMPLE_1.to_string_vector();
		let grid = Grid::from_string_vec(&input);

		println!("{}", grid);
	}

	#[test]
	fn test_find_start() {
		let input_1 = SAMPLE_1.to_string_vector();
		let grid_1 = Grid::from_string_vec(&input_1);
		let point_1 = grid_1.find('S').unwrap_or((-1, -1));

		assert_eq!(point_1, (1, 1));

		let input_2 = SAMPLE_2.to_string_vector();
		let grid_2 = Grid::from_string_vec(&input_2);
		let point_2 = grid_2.find('S').unwrap_or((-1, -1));

		assert_eq!(point_2, (2, 0));
	}
}