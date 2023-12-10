use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use aoc_2023::utils::{self, Grid};

#[derive(PartialEq, Eq, Hash)]
enum Direction {
	North,
	South,
	East,
	West
}

impl Direction {
	fn point(&self) -> Point {
		match self {
			Self::North => Point { x: -1, y: 0 },
			&Self::South => Point { x: 1, y: 0 },
			Self::East => Point { x: 0, y: 1 },
			Self::West => Point { x: 0, y: -1 },
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32
}

impl Add for Point {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		 Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		 }
	}
}

impl AddAssign for Point {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		 self.x += rhs.x;
		 self.y += rhs.y;
	}
}

impl Sub for Point {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl SubAssign for Point {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl Point {
	fn from(value: (i32, i32)) -> Self {
		Self {
			x: value.0,
			y: value.1
		}
	}

	fn move_to(&mut self, direction: &Direction) {
		let point = direction.point();

		*self += point;
	}
}

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
	let grid = utils::Grid::from_string_vec(&input);
	let start = Point::from(grid.find('S').unwrap());
	let distance = traverse_pipes(start, &grid);

	println!("Distance: {}", distance);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::StringExtension;
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
		let grid = utils::Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let distance = traverse_pipes(start, &grid);

		assert_eq!(distance, 4);
	}

	#[test]
	fn test_sample_2() {
		let input = SAMPLE_2.to_string_vector();
		let grid = utils::Grid::from_string_vec(&input);
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
		let grid = utils::Grid::from_string_vec(&input);

		println!("{}", grid);
	}

	#[test]
	fn test_find_start() {
		let input_1 = SAMPLE_1.to_string_vector();
		let grid_1 = utils::Grid::from_string_vec(&input_1);
		let point_1 = grid_1.find('S').unwrap_or((-1, -1));

		assert_eq!(point_1, (1, 1));

		let input_2 = SAMPLE_2.to_string_vector();
		let grid_2 = utils::Grid::from_string_vec(&input_2);
		let point_2 = grid_2.find('S').unwrap_or((-1, -1));

		assert_eq!(point_2, (2, 0));
	}
}