use std::collections::HashSet;

use crate::*;

type Coord = (isize, isize, isize); //x, y, z

fn neighbors(coord: &Coord) -> impl IntoIterator<Item = Coord> {
	vec![
		(coord.0 - 1, coord.1, coord.2),
		(coord.0 + 1, coord.1, coord.2),
		(coord.0, coord.1 - 1, coord.2),
		(coord.0, coord.1 + 1, coord.2),
		(coord.0, coord.1, coord.2 - 1),
		(coord.0, coord.1, coord.2 + 1),
	]
}

pub fn a(input: &str) -> impl Display {
	let coords: HashSet<Coord> = input
		.lines()
		.filter_map(|line| {
			let mut split = line.split(',');
			Some((split.next()?.parse().ok()?, split.next()?.parse().ok()?, split.next()?.parse().ok()?))
		})
		.collect();

	coords.iter().map(|coord| neighbors(coord).into_iter().filter(|n| !coords.contains(n)).count()).sum::<usize>()
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
		//assert_eq!(b(&test_input_as_string(18)).to_string(), "58");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(18)).to_string(), "4444");
		//assert_eq!(b(&input_as_string(18)).to_string(), "");
	}
}
