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

pub trait StringExtension {
	fn to_string_vector(&self) -> Vec<String>;
}

impl StringExtension for str {
	fn to_string_vector(&self) -> Vec<String> {
		self.lines()
			.map(|line| line.trim().to_string())
			.filter(|line| !line.is_empty())
			.collect()
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
	pub x: u32,
	pub y: u32
}
