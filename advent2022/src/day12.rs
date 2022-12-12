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
	
	fn populate_path_lengths_to_end(&self) -> HashMap<Coord, usize> {
		let mut navigation = HashMap::new();
		navigation.insert(self.end, 0);
		
		for steps in 1.. {
			let mut found_anything_new = false;
			for (coord, steps_to_reach) in navigation.clone() {
				if steps_to_reach == steps - 1 {
					let dst = self.get(coord).unwrap();
					
					//check up one row
					let up = (coord.0 - 1, coord.1);
					if self.get(up).filter(|src| can_traverse(*src, dst)).is_some() {
						let previous_steps_to_reach = *navigation.get(&up).unwrap_or(&1000);
						if steps < previous_steps_to_reach {
							found_anything_new |= navigation.insert(up, steps).is_none();
						}
					}
					
					//check down one row
					let down = (coord.0 + 1, coord.1);
					if self.get(down).filter(|src| can_traverse(*src, dst)).is_some() {
						let previous_steps_to_reach = *navigation.get(&down).unwrap_or(&1000);
						if steps < previous_steps_to_reach {
							found_anything_new |= navigation.insert(down, steps).is_none();
						}
					}
					
					//left one column
					let left = (coord.0, coord.1 - 1);
					if self.get(left).filter(|src| can_traverse(*src, dst)).is_some() {
						let previous_steps_to_reach = *navigation.get(&left).unwrap_or(&1000);
						if steps < previous_steps_to_reach {
							found_anything_new |= navigation.insert(left, steps).is_none();
						}
					}
					
					//right one column
					let right = (coord.0, coord.1 + 1);
					if self.get(right).filter(|src| can_traverse(*src, dst)).is_some() {
						let previous_steps_to_reach = *navigation.get(&right).unwrap_or(&1000);
						if steps < previous_steps_to_reach {
							found_anything_new |= navigation.insert(right, steps).is_none();
						}
					}
				}
			}
			
			if !found_anything_new {
				break
			}
		}
		
		navigation
	}
	
	fn find_all(&self, what: char) -> Vec<Coord> {
		let mut result = Vec::new();
		for row in 0..self.rows() {
			for col in 0..self.cols() {
				if let Some(here) = self.get((row as _, col as _)) {
					if here == what {
						result.push((row as _, col as _))
					}
					
				}
			}
		}
		result
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
	let n = map.populate_path_lengths_to_end();
	*n.get(&map.start).unwrap()
}

pub fn b(input: &str) -> impl Display {
	let map = parse(input);
	let starts = map.find_all('a');
	
	let mut shortest_path = 10000usize;
	let navigation = map.populate_path_lengths_to_end();
	
	for start in starts {
		if let Some(path) = navigation.get(&start) {
			shortest_path = shortest_path.min(*path);
		}
	}
	
	shortest_path
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(12)).to_string(), "31");
		assert_eq!(b(&test_input_as_string(12)).to_string(), "29");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(12)).to_string(), "504");
		assert_eq!(b(&input_as_string(12)).to_string(), "500");
	}
}
