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
	rows: usize,
	columns: usize
}

impl Grid {
	pub fn new(rows: usize, columns: usize, default_value: char) -> Self {
		Self {
			data: vec![default_value; rows.checked_mul(columns).unwrap()],
			rows,
			columns
		}
	}

	fn get_index(&self, x: usize, y: usize) -> usize {
		y * self.columns + x
	}

	fn get_value(&self, x: usize, y: usize) -> Option<char> {
		if !(0 .. self.columns).contains(&y) || !(0 .. self.rows).contains(&x) {
			return None
		}

		let index = self.get_index(x, y);
		self.data.get(index).copied()
	}

	fn set_value(&mut self, value: char, x: usize, y: usize) {
		if x >= self.rows || y >= self.columns {
			panic!("Coordinates don't exist ({}, {})", x, y);
		}

		let index = self.get_index(x, y);

		if let Some(cell) = self.data.get_mut(index) {
			*cell = value;
		}
	}
}
