use aoc_2023::utils::grid::Grid;
use aoc_2023::utils::point::Point;
use aoc_2023::utils::utils;

fn find_voids<T, F>(data: T) -> Vec<usize> where T: IntoIterator<Item = F>, F: AsRef<[char]> {
	data.into_iter()
		.enumerate()
		.filter_map(|(index, sequence)| {
			if sequence.as_ref().iter().any(|&cell| cell != '.') {
				None
			} else {
				Some(index)
			}
		})
		.collect()
}

fn expand_universe(universe: &mut Grid) {
	let horizontal_voids = find_voids(universe.iter_rows());
	let vertical_voids = find_voids(universe.iter_columns());

	horizontal_voids.iter().rev().for_each(|&void_index| universe.insert_row_at(void_index, '.'));
	vertical_voids.iter().rev().for_each(|&void_index| universe.insert_column_at(void_index, '.'));
}

fn find_galaxies(universe: &Grid) -> Vec<Point> {
	universe.iter_rows()
		.enumerate()
		.flat_map(|(x, row)| {
			row.iter()
				.enumerate()
				.filter_map(move |(y, &ch)| (ch == '#')
					.then(|| Point::new(x as i32, y as i32)))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn get_galaxy_distances(universe: &Grid) -> Vec<i32> {
	let galaxies = find_galaxies(universe);

	let subtracted: Vec<Point> = galaxies.iter().enumerate()
		.flat_map(|(i, &p1)| galaxies.iter().enumerate()
			.skip(i + 1)
			.map(move |(_, &p2)| p1 - p2 ))
		.collect();

	subtracted.iter().map(|distance| {
		distance.x.abs() + distance.y.abs()
	}).collect::<Vec<_>>()
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day11.txt").unwrap();
	let mut universe = Grid::from_string_vec(&input);
	expand_universe(&mut universe);

	let distances = get_galaxy_distances(&universe);
	let distance_sum = distances.iter().fold(0, |acc, x| acc + x);

	println!("Part 1: {}", distance_sum);
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
}