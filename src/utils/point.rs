use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
	pub x: i64,
	pub y: i64
}


impl Point {
	#[inline]
	#[must_use]
	pub fn new(x: i64, y: i64) -> Self {
		Self { x, y }
	}

	#[inline]
	#[must_use]
	pub fn from(value: (i64, i64)) -> Self {
		Self {
			x: value.0,
			y: value.1
		}
	}

	pub fn move_to(&mut self, direction: &Direction) {
		let point = direction.point();

		*self += point;
	}

	pub fn manhattan_to(&self, other: &Point) -> u64 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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


impl Mul<i64> for Point {
	type Output = Self;

	fn mul(self, scalar: i64) -> Self::Output {
		Self {
			x: self.x * scalar,
			y: self.y * scalar
		}
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}


#[derive(PartialEq, Eq, Hash, Clone)]
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
