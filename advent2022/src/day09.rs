use crate::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
	x: isize,
	y: isize,
}

impl Vec2 {
	fn from_char(c: char) -> Self {
		match c {
			'U' => Vec2 { x: 0, y: 1 },
			'R' => Vec2 { x: 1, y: 0 },
			'D' => Vec2 { x: 0, y: -1 },
			'L' => Vec2 { x: -1, y: 0 },
			_ => panic!("unexpected item in bagging area"),
		}
	}
	
	fn flip(&self) -> Self {
		*self * -1
	}
	
	fn rotate(&self) -> Self {
		Vec2 { x: self.y, y: self.x }
	}
}

impl std::ops::Mul<isize> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: isize) -> Self::Output {
		Vec2 { x: self.x * rhs, y: self.y * rhs }
	}
}

impl std::ops::Add for Vec2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

fn show(head: &Vec2, tail: &Vec2) {
	let xbounds = head.x.min(tail.x.min(0))..(head.x.max(tail.x.max(5)) + 1);
	let ybounds = head.y.min(tail.y.min(0))..(head.y.max(tail.y.max(4)) + 1);
	
	for y in ybounds.rev() {
		for x in xbounds.clone() {
			if x == head.x && y == head.y {
				print!("H");
			} else if x == tail.x && y == tail.y {
				print!("T");
			} else {
				print!("_")
			}
		}
		println!()
	}
}

pub fn a(input: String) -> impl Display {
	let mut head = Vec2 { x: 0, y: 0 };
	let mut tail = Vec2 { x: 0, y: 0 };

	let mut unique_tail_locations = std::collections::HashSet::<Vec2>::new();
	unique_tail_locations.insert(tail);

	for dir in input
		.lines()
		.flat_map(|line| std::iter::repeat(Vec2::from_char(line.chars().next().expect("nonempty"))).take(line[2..].parse::<usize>().expect("numeric")))
	{
		let oldhead = head.clone();
		head = head + dir;
		
		let a = oldhead + dir.flip();
		if tail == a {
			tail = tail + dir; //move in same direction
		} else if tail == a + dir.rotate() {
			tail = tail + dir.rotate().flip() + dir;
		} else if tail == a + dir.rotate().flip() {
			tail = tail + dir.rotate() + dir;
		}
		
		//show(&head, &tail);

		unique_tail_locations.insert(tail);
	}

	unique_tail_locations.len()
}

pub fn b(input: String) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(test_input_as_string(9)).to_string(), "13");
		//assert_eq!(b(test_input_as_string(9)).to_string(), "45000");
	}

	#[test]
	fn real() {
		assert_eq!(a(input_as_string(9)).to_string(), "5907");
		//assert_eq!(b(input_as_string(9)).to_string(), "213958");
	}
}
