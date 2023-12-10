use core::fmt;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_to_vector(file_path: &str) -> Result<Vec<String>, io::Error> {
	let file = File::open(file_path)?;
	let reader = io::BufReader::new(file);

	let lines_vec: Vec<String> = reader
		.lines()
		.map(|line| line.unwrap().trim().to_string())
		.filter(|line| !line.is_empty())
		.collect();

	Ok(lines_vec)
}

pub fn read_file_to_line_number_vector(file_path: &str) -> Result<Vec<Vec<i32>>, io::Error> {
	let file = File::open(file_path)?;
	let reader = io::BufReader::new(file);

	let lines = reader.lines().map(|line| {
			line.map(|l| {
				l.split_whitespace().filter_map(|num| num.parse().ok()).collect()
			})
		}).collect();

	lines
}

pub fn read_file_to_blocks(file_path: &str) -> Result<Vec<Vec<String>>, io::Error> {
	let file = File::open(file_path)?;
	let reader = io::BufReader::new(file);

	let lines_vec: Vec<_> = reader
		.lines()
		.map(|line| line.unwrap().trim().to_string())
		.collect::<Vec<_>>()
		.split(|line| line.trim().is_empty())
		.filter(|block| !block.is_empty())
		.map(|block| block.iter().map(|line| line.trim().to_string()).collect())
		.collect();

	Ok(lines_vec)
}


pub trait StringExtension {
	fn to_string_vector(&self) -> Vec<String>;
	fn to_string_blocks(&self) -> Vec<Vec<String>>;
}

impl StringExtension for str {
	fn to_string_vector(&self) -> Vec<String> {
		self.lines()
			.map(|line| line.trim().to_string())
			.filter(|line| !line.is_empty())
			.collect()
	}

	fn to_string_blocks(&self) -> Vec<Vec<String>> {
		self.lines()
			.map(|line| line.trim().to_string())
			.collect::<Vec<_>>()
			.split(|line| line.trim().is_empty())
			.filter(|block| !block.is_empty())
			.map(|block| block.iter().map(|line| line.trim().to_string()).collect::<Vec<_>>())
			.collect()
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
	pub x: u32,
	pub y: u32
}

#[derive(Debug, Clone)]
pub struct Grid {
	data: Vec<char>,
	pub rows: usize,
	pub columns: usize
}

impl Grid {
	pub fn new(rows: usize, columns: usize, default_value: char) -> Self {
		Self {
			data: vec![default_value; rows.checked_mul(columns).unwrap()],
			rows,
			columns
		}
	}

	pub fn empty() -> Self {
		Self {
			data: vec![],
			rows: 0,
			columns: 0
		}
	}
	pub fn from_string_vec(vector: &Vec<String>) -> Self {
		if vector.is_empty() {
			Self::empty();
		}

		let height = vector.len();
		let width = vector[0].len();
		let cells = vector.into_iter().flat_map(|s| s.chars().collect::<Vec<_>>()).collect();

		Self {
			data: cells,
			rows: height,
			columns: width
		}
	}

	fn get_index(&self, x: usize, y: usize) -> usize {
		x * self.columns + y
	}

	pub fn get_value(&self, x: usize, y: usize) -> Option<char> {
		if !(0 .. self.columns).contains(&y) || !(0 .. self.rows).contains(&x) {
			return None
		}

		let index = self.get_index(x, y);
		self.data.get(index).copied()
	}

	pub fn set_value(&mut self, value: char, x: usize, y: usize) {
		if x >= self.rows || y >= self.columns {
			panic!("Coordinates don't exist ({}, {})", x, y);
		}

		let index = self.get_index(x, y);

		if let Some(cell) = self.data.get_mut(index) {
			*cell = value;
		}
	}

	pub fn find(&self, value: char) -> Option<(i32, i32)> {
		if let Some(index) = self.data.iter().position(|&item| item == value) {
			let x: i32 = (index as i32) / (self.columns as i32);
			let y: i32 = (index as i32) % (self.columns as i32);

			return Some((x, y))
		}

		None
	}
}

impl fmt::Display for Grid {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut buffer = "".to_string();

		for x in 0 .. self.rows {
			for y in 0 .. self.columns {
				buffer.push_str(&self.get_value(x, y).unwrap().to_string()) ;
			}

			buffer.push('\n');
		}

		writeln!(f, "{}", buffer)
	}
}