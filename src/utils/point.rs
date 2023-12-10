use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
	pub x: i32,
	pub y: i32
}


impl Point {
	#[inline]
	#[must_use]
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	#[inline]
	#[must_use]
	pub fn from(value: (i32, i32)) -> Self {
		Self {
			x: value.0,
			y: value.1
		}
	}

	pub fn move_to(&mut self, direction: &Direction) {
		let point = direction.point();

		*self += point;
	}
}


impl Add for Point {
	type Output = Self;

	#[inline]
	#[must_use]
	fn add(self, rhs: Self) -> Self::Output {
		 Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		 }
	}
}


impl AddAssign for Point {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		 self.x += rhs.x;
		 self.y += rhs.y;
	}
}


impl Sub for Point {
	type Output = Self;

	#[inline]
	#[must_use]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}


impl SubAssign for Point {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}


#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
	North,
	South,
	East,
	West
}

impl Direction {
	#[inline]
	#[must_use]
	pub fn point(&self) -> Point {
		match self {
			Self::North => Point::new(-1, 0),
			&Self::South => Point::new(1, 0),
			Self::East => Point::new(0, 1),
			Self::West => Point::new(0, -1)
		}
	}
}
