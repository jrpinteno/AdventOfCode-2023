use aoc_2023::utils::grid::Grid;
use aoc_2023::utils::point::Point;
use aoc_2023::utils::utils;

fn find_voids<T>(data: T) -> Vec<i64> where T: IntoIterator<Item = Vec<char>> {
	data.into_iter()
		.enumerate()
		.filter_map(|(index, sequence)| {
			if sequence.iter().any(|&cell| cell != '.') {
				None
			} else {
				Some(index as i64)
			}
		})
		.collect()
}

fn expand_universe(universe: &mut Grid) {
	let horizontal_voids = find_voids(universe.iter_rows());
	let vertical_voids = find_voids(universe.iter_columns());

	horizontal_voids.iter().rev().for_each(|&void_index| universe.insert_row_at(void_index as usize, '.'));
	vertical_voids.iter().rev().for_each(|&void_index| universe.insert_column_at(void_index as usize, '.'));
}

fn find_galaxies(universe: &Grid) -> Vec<Point> {
	universe
		.find_all('#')
		.iter()
		.map(|&(x, y)| Point::new(x as i64, y as i64))
		.collect()
}

fn get_galaxy_distances(universe: &Grid) -> Vec<u64> {
	let galaxies = find_galaxies(universe);

	galaxies.iter().enumerate()
		.flat_map(|(i, &p1)| galaxies.iter().enumerate()
			.skip(i + 1)
			.map(move |(_, &p2)| {
				p1.manhattan_to(&p2)
			}))
		.collect()
}

fn count_voids_between(points: &(i64, i64), voids: &Vec<i64>) -> u64 {
	let (p1, p2) = points;
	let (min_point, max_point) = (p1.min(&p2), p1.max(&p2));

	voids.iter()
		.filter(|&&x| {
			(min_point .. max_point).contains(&&x)
		})
		.count() as u64
}

fn get_galaxy_distances_expansion(universe: &Grid, expansion_factor: u32) -> Vec<u64> {
	let horizontal_voids = &find_voids(universe.iter_rows());
	let vertical_voids = &find_voids(universe.iter_columns());
	let galaxies = find_galaxies(&universe);

	galaxies.iter().enumerate()
		.flat_map(|(i, &p1)| galaxies.iter().enumerate()
			.skip(i + 1)
			.map(move |(_, &p2)| {
				let h_voids_between = count_voids_between(&(p1.x, p2.x), &horizontal_voids);
				let v_voids_between = count_voids_between(&(p1.y, p2.y), &vertical_voids);

				p1.manhattan_to(&p2) + (h_voids_between + v_voids_between) * (expansion_factor as u64 - 1)
			}))
		.collect::<Vec<_>>()
}

fn part_1(input: &Vec<String>) -> u64 {
	let mut universe = Grid::from_string_vec(&input);
	expand_universe(&mut universe);

	let distances = get_galaxy_distances(&universe);
	distances.iter().fold(0, |acc, x| acc + x)
}

fn part_2(input: &Vec<String>, expansion_factor: u32) -> u64 {
	let universe = Grid::from_string_vec(&input);

	let distances = get_galaxy_distances_expansion(&universe, expansion_factor);
	distances.iter().fold(0, |acc, &x| acc + x )
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day11.txt").unwrap();
	let result_1 = part_1(&input);

	println!("Part 1: {}", result_1);

	let result_2 = part_2(&input, 1_000_000);
	println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::grid::Grid;
	use aoc_2023::utils::utils::StringExtension;
	use super::*;

	const SAMPLE_1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

	#[test]
	fn test_sample_1() {
		let input = SAMPLE_1.to_string_vector();
		let mut universe = Grid::from_string_vec(&input);
		expand_universe(&mut universe);

		let distances = get_galaxy_distances(&universe);
		let distance_sum = distances.iter().fold(0, |acc, x| acc + x);

		assert_eq!(distance_sum, 374);
	}

	#[test]
	fn test_grid_iter() {
		let test = "\
123
456
789";

		let input = test.to_string_vector();
		let mut grid = Grid::from_string_vec(&input);

		for row in grid.iter_rows() {
			println!("{:?}", row);
		}

		for column in grid.iter_columns() {
			println!("{:?}", column);
		}

		grid.insert_row_at(1, '.');
		println!("{}", grid);

		grid.insert_column_at(1, '.');
		println!("{}", grid);
	}

	#[test]
	fn test_find_voids_expand() {
		let input = SAMPLE_1.to_string_vector();
		let mut universe = Grid::from_string_vec(&input);

		println!("{}", universe);

		expand_universe(&mut universe);

		println!("{}", universe);
		find_galaxies(&universe);
	}

	#[test]
	fn test_find_distance() {
		let input = SAMPLE_1.to_string_vector();
		let mut universe = Grid::from_string_vec(&input);
		expand_universe(&mut universe);
		let galaxies = find_galaxies(&universe);

		let grid_distance_17 = galaxies[0] - galaxies[6];
		assert_eq!(grid_distance_17.x.abs() + grid_distance_17.y.abs(), 15);

		let grid_distance_36 = galaxies[2] - galaxies[5];
		assert_eq!(grid_distance_36.x.abs() + grid_distance_36.y.abs(), 17);

		let grid_distance_89 = galaxies[7] - galaxies[8];
		assert_eq!(grid_distance_89.x.abs() + grid_distance_89.y.abs(), 5);
	}

	#[test]
	fn test_find_distance_2() {
		let input = SAMPLE_1.to_string_vector();
		let result = part_2(&input, 2);

		assert_eq!(result, 374);
	}

	#[test]
	fn test_find_distance_10() {
		let input = SAMPLE_1.to_string_vector();
		let result = part_2(&input, 10);

		assert_eq!(result, 1030);
	}

	#[test]
	fn test_find_distance_100() {
		let input = SAMPLE_1.to_string_vector();
		let result = part_2(&input, 100);

		assert_eq!(result, 8410);
	}
}
