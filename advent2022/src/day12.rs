#![allow(clippy::cast_sign_loss)]

use std::collections::HashSet;

use crate::*;

type Coord = (isize, isize); //(row, col)

struct Map {
	map: Vec<Vec<char>>,
	start: Coord,
	end: Coord,
}

impl Map {
	fn get(&self, index: Coord) -> Option<char> {
		if let Some(row) = self.map.get(index.0 as usize) {
			row.get(index.1 as usize).copied()
		} else {
			None
		}
	}

	fn path(&self, mut stopping_condition: impl FnMut(&Coord) -> bool) -> Option<usize> {
		let mut explored_area = HashSet::new();
		let mut to_explore = HashSet::new();
		to_explore.insert(self.end);
		let mut frontier_of_exploration = HashSet::new();

		let mut steps = 0;
		while !&to_explore.is_empty() {
			steps += 1;

			for &coord in &to_explore {
				let dst = self.get(coord).unwrap();
				for src_coord in [(coord.0 - 1, coord.1), (coord.0 + 1, coord.1), (coord.0, coord.1 - 1), (coord.0, coord.1 + 1)]
					.iter()
					.filter(|&c| !explored_area.contains(c) && self.get(*c).filter(|src| (*src as u8 + 1) >= (dst as u8)).is_some())
				{
					if stopping_condition(src_coord) {
						return Some(steps);
					}
					frontier_of_exploration.insert(*src_coord);
				}
			}

			for coord in &frontier_of_exploration {
				explored_area.insert(*coord);
			}

			std::mem::swap(&mut to_explore, &mut frontier_of_exploration);
			frontier_of_exploration.clear();
		}

		None
	}
}

fn parse(input: &str) -> Map {
	Map {
		map: input
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
			.collect(),
		start: {
			let (row_id, row) = input.lines().enumerate().find(|(_, row)| row.contains('S')).expect("start exists");
			let (col_id, _) = row.chars().enumerate().find(|(_, c)| *c == 'S').unwrap();
			(row_id.try_into().expect("start row fits in isize"), col_id.try_into().expect("start col fits in isize"))
		},
		end: {
			let (row_id, row) = input.lines().enumerate().find(|(_, row)| row.contains('E')).expect("end exists");
			let (col_id, _) = row.chars().enumerate().find(|(_, c)| *c == 'E').unwrap();
			(row_id.try_into().expect("end row fits in isize"), col_id.try_into().expect("end col fits in isize"))
		},
	}
}

pub fn a(input: &str) -> impl Display {
	let map = parse(input);
	map.path(|coord| &map.start == coord).expect("start reachable from end")
}

pub fn b(input: &str) -> impl Display {
	let map = parse(input);
	map.path(|coord| matches!(map.get(*coord), Some(height) if height == 'a')).expect("at least one start reachable from end")
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
