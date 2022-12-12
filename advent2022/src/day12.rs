use std::collections::HashMap;

use crate::*;

type Coord = (isize, isize);
struct Map {
	map: Vec<Vec<char>>,
	start: Coord,
	end: Coord
}

impl Map {
	fn rows(&self) -> usize {
		self.map.len()
	}

	fn cols(&self) -> usize {
		self.map[0].len()
	}
	
	fn get(&self, index: Coord) -> Option<char> {
		if let Some(row) = self.map.get(index.0 as usize) {
			row.get(index.1 as usize).copied()
		} else {
			None
		}
	}
}

fn parse(input: &str) -> Map {
	let (start_row_id, start_row) = input.lines().enumerate().find(|(_, row)| row.contains('S')).unwrap();
	let (start_col_id, _) = start_row.chars().enumerate().find(|(_, c)| *c == 'S').unwrap();

	let (end_row_id, end_row) = input.lines().enumerate().find(|(_, row)| row.contains('E')).unwrap();
	let (end_col_id, _) = end_row.chars().enumerate().find(|(_, c)| *c == 'E').unwrap();

	let map: Vec<Vec<char>> = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'S' => 'a',
					'E' => 'z',
					etc => etc,
				})
				.collect()
		})
		.collect();

	Map { map, start: (start_row_id as isize, start_col_id as isize), end: (end_row_id as isize, end_col_id as isize) }
}

fn can_traverse(src: char, dst: char) -> bool {
	(src as u8 + 1) >= (dst as u8)
}

pub fn a(input: &str) -> impl Display {
	let map = parse(input);
	let mut navigation: HashMap<Coord, usize> = HashMap::new();
	navigation.insert(map.end, 0);

	for steps in 1.. {
		for (coord, steps_to_reach) in navigation.clone() {
			if steps_to_reach == steps - 1 {
				let dst = map.get(coord).unwrap();
				
				//check up one row
				let up = (coord.0 - 1, coord.1);
				if map.get(up).filter(|src| can_traverse(*src, dst)).is_some() {
					let previous_steps_to_reach = *navigation.get(&up).unwrap_or(&1000);
					if steps < previous_steps_to_reach {
						navigation.insert(up, steps);
					}
				}
				
				//check down one row
				let down = (coord.0 + 1, coord.1);
				if map.get(down).filter(|src| can_traverse(*src, dst)).is_some() {
					let previous_steps_to_reach = *navigation.get(&down).unwrap_or(&1000);
					if steps < previous_steps_to_reach {
						navigation.insert(down, steps);
					}
				}
				
				//left one column
				let left = (coord.0, coord.1 - 1);
				if map.get(left).filter(|src| can_traverse(*src, dst)).is_some() {
					let previous_steps_to_reach = *navigation.get(&left).unwrap_or(&1000);
					if steps < previous_steps_to_reach {
						navigation.insert(left, steps);
					}
				}
				
				//right one column
				let left = (coord.0, coord.1 + 1);
				if map.get(left).filter(|src| can_traverse(*src, dst)).is_some() {
					let previous_steps_to_reach = *navigation.get(&left).unwrap_or(&1000);
					if steps < previous_steps_to_reach {
						navigation.insert(left, steps);
					}
				}
			}
		}

		if navigation.contains_key(&map.start) {
			return navigation.get(&map.start).unwrap().to_string();
		}
	}

	unreachable!()
}

pub fn b(input: &str) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(12)).to_string(), "31");
		assert_eq!(b(&test_input_as_string(12)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(12)).to_string(), "504");
		assert_eq!(b(&input_as_string(12)).to_string(), "x");
	}
}
