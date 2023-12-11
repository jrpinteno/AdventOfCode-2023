use core::fmt;

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

	pub fn empty() -> Self {
		Self {
			data: vec![],
			rows: 0,
			columns: 0
		}
	}

	pub fn empty_copy_with_default(&self, default_value: char) -> Self {
		Grid::new(self.rows, self.columns, default_value)
	}

	#[must_use]
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

	pub fn get_size(&self) -> (usize, usize) {
		(self.columns, self.rows)
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

	pub fn find_all(&self, value: char) -> Vec<(i32, i32)> {
		self.iter_rows()
			.enumerate()
			.flat_map(|(x, row)| {
				row.iter()
					.enumerate()
					.filter_map(move |(y, &c)| (c == value)
						.then(|| (x as i32, y as i32)))
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>()
	}

	// Custom iterator for iterating over columns
	pub fn iter_columns(&self) -> impl Iterator<Item = Vec<char>> + '_ {
		(0..self.columns).map(move |col| {
			(0..self.rows).map(move |row| self.data[row * self.columns + col]).collect()
		})
	}

	// Custom iterator for iterating over rows
	pub fn iter_rows(&self) -> impl Iterator<Item = Vec<char>> + '_ {
		(0..self.rows).map(move |row| {
			self.data[row * self.columns..(row + 1) * self.columns].to_vec()
		})
	}
}


impl Grid {
	pub fn insert_row_at(&mut self, position: usize, default_value: char) {
		if position > self.rows {
			panic!("Invalid row insertion position");
		}

		let start_index = position * self.columns;

		self.data.splice(start_index .. start_index, std::iter::repeat(default_value).take(self.columns));
		self.rows += 1;
	}

	pub fn insert_column_at(&mut self, position: usize, default_value: char) {
		if position > self.columns {
			panic!("Invalid column insertion position");
		}

		for row in (0 .. self.rows).rev() {
			let insert_index = row * self.columns + position;
			self.data.insert(insert_index, default_value);
		}

		self.columns += 1;
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