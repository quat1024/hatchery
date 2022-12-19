use std::collections::HashSet;

use crate::*;

type Coord = (isize, isize, isize); //x, y, z

pub fn a(input: &str) -> impl Display {
	let coords: HashSet<Coord> = input.lines().filter_map(|line| {
		let mut split = line.split(',');
		Some((split.next()?.parse().ok()?, split.next()?.parse().ok()?, split.next()?.parse().ok()?))
	}).collect();
	
	coords.iter().map(|coord| {
		let mut score = 6;
		if coords.contains(&(coord.0 - 1, coord.1, coord.2)) {
			score -= 1;
		}
		if coords.contains(&(coord.0 + 1, coord.1, coord.2)) {
			score -= 1;
		}
		if coords.contains(&(coord.0, coord.1 - 1, coord.2)) {
			score -= 1;
		}
		if coords.contains(&(coord.0, coord.1 + 1, coord.2)) {
			score -= 1;
		}
		if coords.contains(&(coord.0, coord.1, coord.2 - 1)) {
			score -= 1;
		}
		if coords.contains(&(coord.0, coord.1, coord.2 + 1)) {
			score -= 1;
		}
		score
	}).sum::<isize>()
}

pub fn b(input: &str) -> impl Display {
	""
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(18)).to_string(), "64");
		//assert_eq!(b(&test_input_as_string(18)).to_string(), "");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(18)).to_string(), "??");
		//assert_eq!(b(&input_as_string(18)).to_string(), "");
	}
}