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
