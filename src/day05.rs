use std::ops::Range;

use aoc_2023::utils;

#[derive(Debug, PartialEq)]
struct SeedMap {
	source_range: Range<u64>,
	offset: u64,
	destination_start: u64
}

impl SeedMap {
	fn from_string(map: String) -> Self {
		let parsed: Vec<_> = map
			.split_whitespace()
			.filter_map(|number| number.parse::<u64>().ok())
			.collect();

		Self {
			source_range: parsed[1] .. parsed[1] + parsed[2],
			offset: parsed[2],
			destination_start: parsed[0]
		}
	}
}

trait Intersect {
	fn intersect_range(&self, other: &Range<u64>) -> Option<(u64, u64)>;
	fn intersect_points(&self, other: &Range<u64>) -> Option<Vec<u64>>;
}

impl Intersect for Range<u64> {
	fn intersect_range(&self, other: &Range<u64>) -> Option<(u64, u64)> {
		let max_start = self.start.max(other.start);
		let min_end = (self.end - 1).min(other.end - 1);

		if max_start > min_end {
			return None;
		}

		Some((max_start, min_end))
	}

	fn intersect_points(&self, other: &Range<u64>) -> Option<Vec<u64>> {
		if let Some(range) = self.intersect_range(other) {
			let mut points = Vec::new();
			for i in range.0 .. range.1 {
				points.push(i);
			}

			return Some(points)
		}

		None
	}
}

fn parse_seeds(seeds: &String) -> Vec<u64> {
	seeds
		.split(":")
		.nth(1)
		.unwrap_or("")
		.split_whitespace()
		.filter_map(|number| number.parse::<u64>().ok())
		.collect()
}

fn parse_seeds_almanac(seed_blocks: Vec<Vec<String>>) -> (Vec<u64>, Vec<Vec<SeedMap>>) {
	let mut blocks_iter = seed_blocks.iter();
	let seed_block = blocks_iter.next().unwrap().first().unwrap();
	let seeds: Vec<_> = parse_seeds(seed_block);

	let almanac: Vec<Vec<SeedMap>> = blocks_iter
		.map(|block| {
			block
				.iter()
				.skip(1)
				.map(|instruction| SeedMap::from_string(instruction.to_string()))
				.collect()
		}).collect();

	(seeds, almanac)
}

fn get_lowest_location(seeds: &Vec<u64>, almanac: &Vec<Vec<SeedMap>>) -> u64 {
	seeds.iter().map(|seed| {
		let mut location = *seed;

		for seed_maps in almanac {
			for seed_map in seed_maps {
				if seed_map.source_range.contains(&location) {
					location = seed_map.destination_start + location - seed_map.source_range.start;

					break;
				} else {
					location
				};
			}
		}

		location
	}).min().unwrap()
}

fn get_seed_ranges(seeds: Vec<u64>) -> Vec<Range<u64>> {
	seeds.chunks(2).into_iter()
		.filter_map(|chunk| {
			Some(chunk[0] .. chunk[0] + chunk[1])
		})
		.collect()
}

fn find_intersection(seeds: Vec<Range<u64>>, seed_map_range: &Vec<SeedMap>) -> Vec<u64> {
	let intersections = seeds
		.iter()
		.flat_map(|r1| {
			seed_map_range.iter().filter_map(move |r2| {
				if let Some(points) = r1.intersect_points(&r2.source_range) {
					Some(points)
				} else {
					None
				}
			})
		})
		.flatten()
		.collect::<std::collections::BTreeSet<u64>>().into_iter().collect();

	intersections
}

fn main() {
	let seed_blocks = utils::read_file_to_blocks("input/2023/day5.txt").unwrap();
	let (seeds, almanac) = parse_seeds_almanac(seed_blocks);
	let lowest_location = get_lowest_location(&seeds, &almanac);
	println!("Minimum 1: {:?}", lowest_location);

	let seed_ranges = get_seed_ranges(seeds);
	let new_seeds = find_intersection(seed_ranges, almanac.first().unwrap());
	let lowest_location = get_lowest_location(&new_seeds, &almanac);

	println!("Minimum 2: {:?}", lowest_location);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::StringExtension;
	use super::*;

	const SEED_MAP: SeedMap = SeedMap {
		source_range: 50 .. 98,
		offset: 48,
		destination_start: 52,
	};

	const SEED_MAP_STR: &str = "52 50 48";

	const SAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

	#[test]
	fn test_sample_1() {
		let blocks: Vec<_> = SAMPLE.to_string_blocks();
		let (seeds, almanac) = parse_seeds_almanac(blocks);

		let lowest_location = get_lowest_location(&seeds, &almanac);

		assert_eq!(lowest_location, 35);
	}

	#[test]
	fn test_sample_2() {
		let blocks: Vec<_> = SAMPLE.to_string_blocks();
		let (seeds, almanac) = parse_seeds_almanac(blocks);
		let seed_ranges = get_seed_ranges(seeds);
		let new_seeds = find_intersection(seed_ranges, almanac.first().unwrap());

		let lowest_location = get_lowest_location(&new_seeds, &almanac);

		assert_eq!(lowest_location, 46);
	}

	#[test]
	fn test_blocks_count() {
		let blocks: Vec<_> = SAMPLE.to_string_blocks();

		assert_eq!(blocks.len(), 8)
	}

	#[test]
	fn test_blocks_parse_seeds() {
		let blocks: Vec<_> = SAMPLE.to_string_blocks();
		let mut blocks_iter = blocks.iter();
		let seed_block = blocks_iter.next().unwrap().first().unwrap();

		assert_eq!(seed_block, "seeds: 79 14 55 13");
		let seeds: Vec<u64> = parse_seeds(seed_block);

		assert_eq!(seeds.contains(&79), true);
	}

	#[test]
	fn test_parse_seed_map() {
		let map = SeedMap::from_string(SEED_MAP_STR.to_string());

		assert_eq!(map, SEED_MAP);
	}

	#[test]
	fn test_intersection() {
		let range_1 = 12 .. 19u64;
		let range_2 = 8 .. 16u64;
		let range_3 = 4 .. 9u64;

		let result = range_1.intersect_points(&range_2);
		let single_point_intersect = range_2.intersect_points(&range_3);

		let unwrapped = result.unwrap();
		assert_eq!(unwrapped.len(), 2);
		assert_eq!(unwrapped, vec![12, 15]);

		assert_eq!(single_point_intersect.unwrap().len(), 1);
	}

	#[test]
	fn test_intersection_seed_map_range() {
		let seed_ranges = vec![12 .. 27u64, 7 .. 9u64, 1 .. 4u64];
		let map_ranges = vec![8 .. 13u64, 26 .. 29u64, 2 .. 3u64];

		let intersect = find_intersection(seed_ranges, map_ranges);
		println!("{:?}", intersect);
	}
}