use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;

use aoc_2023::utils;

#[derive(Debug)]
struct Node {
	left: String,
	right: String
}

fn parse_input(input: &Vec<String>) -> (VecDeque<char>, HashMap<String, Node>){
	let mut input_iter = input.iter();
	let next = input_iter.next().unwrap();
	let mut instructions = VecDeque::from(next.chars().collect::<Vec<_>>());

	let map: HashMap<String, Node> = input_iter.map(|map_step| {
		let (key, value) = map_step.split_once(" = ").unwrap();
		let (left, right) = value
			.trim_matches(|p| p == '(' || p == ')')
			.split_once(", ")
			.unwrap();

		(
			key.to_string(),
			Node {
				left: left.to_string(),
				right: right.to_string()
			}
		)
	}).collect();

	(instructions, map)
}

fn get_next_step(map: &HashMap<String, Node>, step: &String, instruction: char) -> String {
	let next_node = map.get(step).unwrap();

	match instruction {
		'L' => next_node.left.clone(),
		'R' => next_node.right.clone(),
		_ => panic!("Unknown instruction")
	}
}

fn count_steps(instructions: &mut VecDeque<char>, map: &HashMap<String, Node>) -> u32 {
	let mut steps = 0;
	let mut found = false;
	let mut next_step = "AAA".to_string();

	while !found {
		steps += 1;
		let instruction = instructions.pop_front().unwrap();
		next_step = get_next_step(map, &next_step, instruction);

		found = next_step == "ZZZ";

		if !found {
			instructions.push_back(instruction)
		}
	}

	steps
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let temp = a;
		a = b;
		b = temp % b;
	}

	a
}
fn count_ghost_steps(instructions: &mut VecDeque<char>, map: &HashMap<String, Node>) -> u64 {
	let mut ghost_steps: Vec<_> = map
		.keys()
		.filter(|&initial| initial.ends_with('A'))
		.cloned()
		.collect();

	let steps: Vec<_> = ghost_steps.iter().map(|ghost| {
		let mut steps: u64 = 0;
		let mut next_step = ghost.clone();
		let mut found = false;
		let mut ghost_instructions = instructions.clone();

		while !found {
			steps += 1;
			let instruction = ghost_instructions.pop_front().unwrap();
			next_step = get_next_step(map, &next_step, instruction);
			ghost_instructions.push_back(instruction);

			found = next_step.ends_with('Z');
		}

		steps
	}).collect();

	steps.iter().cloned().fold(1, |acc, x| acc * x / gcd(acc, x))
}

fn main() {
	let input = utils::read_file_to_vector("input/2023/day8.txt").unwrap();
	let (instructions, map) = parse_input(&input);
	let steps = count_steps(&mut instructions.clone(), &map);
	println!("{}", steps);

	let ghost_steps = count_ghost_steps(&mut instructions.clone(), &map);
	println!("{}", ghost_steps);
}

#[cfg(test)]
mod tests {
	use aoc_2023::utils::StringExtension;
	use super::*;

	const SAMPLE_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

	const SAMPLE_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

	const SAMPLE_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

	#[test]
	fn test_sample_1() {
		let input = SAMPLE_1.to_string_vector();

		let (mut instructions, map) = parse_input(&input);
		let steps = count_steps(&mut instructions, &map);
		println!("{}", steps);

		assert_eq!(steps, 2);
	}

	#[test]
	fn test_sample_2() {
		let input = SAMPLE_2.to_string_vector();

		let (mut instructions, map) = parse_input(&input);
		let steps = count_steps(&mut instructions, &map);
		println!("{}", steps);

		assert_eq!(steps, 6);
	}

	#[test]
	fn test_sample_3() {
		let input = SAMPLE_3.to_string_vector();

		let (mut instructions, map) = parse_input(&input);
		let steps = count_ghost_steps(&mut instructions, &map);

		println!("{}", steps);
	}
}