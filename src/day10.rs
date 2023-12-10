use std::collections::HashMap;

use aoc_2023::utils::utils;
use aoc_2023::utils::{point::*, grid::*};


fn get_first_directions(start: Point, grid: &Grid) -> Vec<Direction> {
	let map = HashMap::from([
		(Direction::South, "|LJ"),
		(Direction::North, "|FJ"),
		(Direction::West, "-LF"),
		(Direction::East, "-7J"),
	]);

	// let mut directions = vec![];

	// for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
	// 	let new_point = start + direction.point();

	// 	if let Some(value) = grid.get_value(new_point.x as usize, new_point.y as usize) {
	// 		let is_valid = map.get(&direction).unwrap().contains(value);

	// 		if is_valid {
	// 			directions.push(direction);
	// 		}
	// 	}
	// }

	// directions

	[Direction::North, Direction::East, Direction::South, Direction::West]
		.iter()
		.filter_map(|direction| {
			let new_point = start + direction.point();

			grid.get_value(new_point.x as usize, new_point.y as usize)
				.and_then(|value| {
					map.get(&direction).map(|&valid_chars| valid_chars.contains(value))
	 			})
				.map(|is_valid| if is_valid { Some(direction) } else { None })
				.flatten()
		})
		.cloned()
		.collect::<Vec<Direction>>()
}

fn record_trip(value: char, log_grid: &mut Grid, position: &Point) {
	// Use map for nice print on console :D
	let map: HashMap<char, char> = HashMap::from([
		('L', '└'),
		('J', '┘'),
		('7', '┐'),
		('F', '┌'),
		('|', '│'),
		('-', '─')
	]);

	log_grid.set_value(value, position.x as usize, position.y as usize);
}

fn get_char_for_directions(lhs: Direction, rhs: Direction) -> char {
	match (lhs, rhs) {
		(Direction::North, Direction::South) | (Direction::South, Direction::North) => '|',
		(Direction::North, Direction::East) | (Direction::East, Direction::North) => 'L',
		(Direction::North, Direction::West) | (Direction::West, Direction::North) => 'J',
		(Direction::South, Direction::East) | (Direction::East, Direction::South) => 'F',
		(Direction::South, Direction::West) | (Direction::West, Direction::South) => '7',
		(Direction::East, Direction::West) | (Direction::West, Direction::East) => '-',
		_ => panic!("Wrong direction set")
	}
}

fn count_inner_tiles(log_grid: &mut Grid) -> i32 {
	let mut within_loop = false;
	let mut tiles_loop = 0;

	for i in 0 .. log_grid.get_size().1 {
		for j in 0 .. log_grid.get_size().0 {
			let value = log_grid.get_value(i, j).unwrap();

			match value {
				 '7' | 'F' | '|' => within_loop = !within_loop,
				 'L' | 'J' |'-' => continue,
				'.' => if within_loop {
					tiles_loop += 1;
					log_grid.set_value('I', i, j);
				} else {
					log_grid.set_value('O', i, j);
				},
				_ => panic!("Oops")
			}
		}
	}

	tiles_loop
}

fn get_next_direction(value: char, current_direction: Direction) -> Direction {
	match value {
		 'L' => if current_direction == Direction::South { Direction::East } else { Direction::North },
		 'J' => if current_direction == Direction::East { Direction::North } else { Direction::West },
		 '7' => if current_direction == Direction::East { Direction::South } else { Direction::West },
		 'F' => if current_direction == Direction::North { Direction::East } else { Direction::South },
		 '|' | '-' => current_direction,
		 _ => panic!("Oops"),
	}
}

fn traverse_pipes(start: Point, grid: &Grid) -> (i32, i32) {
	let mut log_grid = grid.empty_copy_with_default('.');
	log_grid.set_value('S', start.x as usize, start.y as usize);

	let first_directions = get_first_directions(start, &grid);

	let mut current_location = start.clone();
	let mut direction = first_directions[0].clone();
	current_location.move_to(&direction);

	let mut distance = 1;

	while start != current_location {
		let value = grid.get_value(current_location.x as usize, current_location.y as usize).unwrap();
		record_trip(value, &mut log_grid, &current_location);

		direction = get_next_direction(value, direction);
		current_location.move_to(&direction);

		distance += 1;
	}

	record_trip(
		get_char_for_directions(first_directions[0].clone(), first_directions[1].clone()),
		&mut log_grid,
		&current_location);

	let tiles_loop = count_inner_tiles(&mut log_grid);

	(distance / 2, tiles_loop)
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day10.txt").unwrap();
	let grid = Grid::from_string_vec(&input);
	let start = Point::from(grid.find('S').unwrap());
	let result = traverse_pipes(start, &grid);

	println!("Distance: {}", result.0);
	println!("Tiles: {}", result.1);
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

	const SAMPLE_3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

	const SAMPLE_4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

	const SAMPLE_5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

	#[test]
	fn test_sample_1() {
		let input = SAMPLE_1.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let distance = traverse_pipes(start, &grid).0;

		assert_eq!(distance, 4);
	}

	#[test]
	fn test_sample_2() {
		let input = SAMPLE_2.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let distance = traverse_pipes(start, &grid).0;

		assert_eq!(distance, 8);
	}

	#[test]
	fn test_sample_3() {
		let input = SAMPLE_3.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let tiles = traverse_pipes(start, &grid).1;

		assert_eq!(tiles, 4);
	}

	#[test]
	fn test_sample_4() {
		let input = SAMPLE_4.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let tiles = traverse_pipes(start, &grid).1;

		assert_eq!(tiles, 8);
	}

	#[test]
	fn test_sample_5() {
		let input = SAMPLE_5.to_string_vector();
		let grid = Grid::from_string_vec(&input);
		let start = Point::from(grid.find('S').unwrap());
		let tiles = traverse_pipes(start, &grid).1;

		assert_eq!(tiles, 10);
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