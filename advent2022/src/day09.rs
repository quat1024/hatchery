use crate::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Vec2 {
	x: isize,
	y: isize,
}

impl Vec2 {
	fn new(x: isize, y: isize) -> Self {
		Vec2 { x, y }
	}

	fn from_char(c: char) -> Self {
		match c {
			'U' => Self::new(0, 1),
			'R' => Self::new(1, 0),
			'D' => Self::new(0, -1),
			'L' => Self::new(-1, 0),
			_ => panic!("unexpected item in bagging area"),
		}
	}

	fn manhattan_dist(self, other: Self) -> usize {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}
}

impl std::ops::Add for Vec2 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

///flattens "R 4" into "R, R, R, R" while parsing
fn parse(input: &str) -> impl Iterator<Item = Vec2> + '_ {
	input.lines().flat_map(|line| std::iter::repeat(Vec2::from_char(line.chars().next().expect("nonempty"))).take(line[2..].parse::<usize>().expect("numeric")))
}

fn update_tail(head: Vec2, tail: Vec2) -> Vec2 {
	if (head.x.abs_diff(tail.x) <= 1) && (head.y.abs_diff(tail.y) <= 1) {
		return tail; //no need to move
	}

	//Minimize distance to the head
	let mut steps =
		vec![Vec2::new(0, 1), Vec2::new(1, 1), Vec2::new(1, 0), Vec2::new(1, -1), Vec2::new(0, -1), Vec2::new(-1, -1), Vec2::new(-1, 0), Vec2::new(-1, 1)];
	steps.sort_by_key(|step| (tail + *step).manhattan_dist(head));
	tail + steps[0]
}

fn drag_rope<const ROPE_LENGTH: usize>(steps: impl Iterator<Item = Vec2>) -> usize {
	assert!(ROPE_LENGTH >= 2, "nontrivial rope");

	let mut rope = [Vec2::default(); ROPE_LENGTH];
	let mut unique_tail_locations = std::collections::HashSet::<Vec2>::new();
	unique_tail_locations.insert(rope[ROPE_LENGTH - 1]);

	for step in steps {
		rope[0] = rope[0] + step;
		for i in 1..ROPE_LENGTH {
			rope[i] = update_tail(rope[i - 1], rope[i]);
		}

		unique_tail_locations.insert(rope[ROPE_LENGTH - 1]);
	}

	unique_tail_locations.len()
}

pub fn a(input: &str) -> impl Display {
	drag_rope::<2>(parse(input))
}

pub fn b(input: &str) -> impl Display {
	drag_rope::<10>(parse(input))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(9)).to_string(), "13");
		assert_eq!(b(&test_input_as_string(9)).to_string(), "1");
		assert_eq!(b(&gimme_input("09 small 2.txt")).to_string(), "36");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(9)).to_string(), "5907");
		assert_eq!(b(&input_as_string(9)).to_string(), "2303");
	}
}

// fn show(head: &Vec2, tail: &Vec2) {
// 	let xbounds = head.x.min(tail.x.min(0))..(head.x.max(tail.x.max(5)) + 1);
// 	let ybounds = head.y.min(tail.y.min(0))..(head.y.max(tail.y.max(4)) + 1);

// 	for y in ybounds.rev() {
// 		for x in xbounds.clone() {
// 			if x == head.x && y == head.y {
// 				print!("H");
// 			} else if x == tail.x && y == tail.y {
// 				print!("T");
// 			} else {
// 				print!("_")
// 			}
// 		}
// 		println!()
// 	}
// 	println!();
// }
