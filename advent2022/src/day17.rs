use std::collections::HashMap;
use std::fmt::Write;

use crate::*;

type Coord = (isize, isize); //column, row

#[derive(Copy, Clone, Debug)]
enum RockShape {
	Flat,
	Plus,
	BackwardL,
	I,
	O,
}

impl RockShape {
	fn cycle() -> impl Iterator<Item = Self> {
		[RockShape::Flat, RockShape::Plus, RockShape::BackwardL, RockShape::I, RockShape::O].iter().copied().cycle()
	}

	//(column, row)
	fn cells(self) -> impl Iterator<Item = Coord> {
		match self {
			RockShape::Flat => [(0, 0), (1, 0), (2, 0), (3, 0)].iter().copied(),
			RockShape::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].iter().copied(),
			RockShape::BackwardL => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].iter().copied(),
			RockShape::I => [(0, 0), (0, 1), (0, 2), (0, 3)].iter().copied(),
			RockShape::O => [(0, 0), (1, 0), (0, 1), (1, 1)].iter().copied(),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum HitResult {
	Air,
	RockOrGround,
	Wall,
}

impl HitResult {
	fn is_air(self) -> bool {
		self == HitResult::Air
	}
}

#[derive(Default, Clone)]
struct Well {
	columns: [HashMap<isize, bool>; 7],
}

impl Well {
	fn get_column(&self, column: isize) -> &HashMap<isize, bool> {
		&self.columns[TryInto::<usize>::try_into(column).unwrap()]
	}

	fn get_column_mut(&mut self, column: isize) -> &mut HashMap<isize, bool> {
		&mut self.columns[TryInto::<usize>::try_into(column).unwrap()]
	}

	fn max_height(&self) -> isize {
		*self.columns.iter().map(|column| column.keys().max().unwrap_or(&-1)).max().unwrap_or(&-1)
	}

	fn hit_test(&self, coord: Coord) -> HitResult {
		if !(0..=6).contains(&coord.0) {
			HitResult::Wall
		} else if coord.1 < 0 || self.get_column(coord.0).contains_key(&coord.1) {
			HitResult::RockOrGround
		} else {
			HitResult::Air
		}
	}

	fn hit_test_rock(&self, shape: RockShape, coord: Coord) -> HitResult {
		for rock_cell in shape.cells() {
			match self.hit_test((coord.0 + rock_cell.0, coord.1 + rock_cell.1)) {
				HitResult::Air => {},
				HitResult::RockOrGround => return HitResult::RockOrGround,
				HitResult::Wall => return HitResult::Wall,
			}
		}
		HitResult::Air
	}

	fn paste_rock(&mut self, shape: RockShape, coord: Coord) {
		for rock_cell in shape.cells() {
			if let Some(old) = self.get_column_mut(coord.0 + rock_cell.0).insert(coord.1 + rock_cell.1, true) {
				assert!(!old, "paste_rock overwrote a rock while pasting {shape:?} at {coord:?}, well state \n{self}");
			}
		}
	}
}

impl Display for Well {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in (0..=self.max_height() + 2).rev() {
			f.write_char('|')?;

			for col in 0..7 {
				f.write_char(if *self.get_column(col).get(&row).unwrap_or(&false) { '#' } else { '.' })?;
			}

			f.write_fmt(format_args!("| {row}\n"))?;
		}
		f.write_str("+0123456+\n")?;
		Ok(())
	}
}

pub fn a(input: &str) -> impl Display {
	let mut gusts = input.lines().next().unwrap().trim().chars().cycle();
	let mut rocks = RockShape::cycle();

	let mut well = Well::default();
	'next_rock: for i in 1..=2022 {
		let rock = rocks.next().unwrap();
		
		let mut rock_coord = (2, well.max_height() + 4);
		
		loop {
			let blow_offset = match gusts.next() {
				Some('<') => -1,
				Some('>') => 1,
				_ => panic!("unexpected item in bagging area"),
			};
			
			let blown_rock_coord = (rock_coord.0 + blow_offset, rock_coord.1);
			if well.hit_test_rock(rock, blown_rock_coord).is_air() {
				rock_coord = (blown_rock_coord.0, blown_rock_coord.1);
			}
			
			let rock_coord_below = (rock_coord.0, rock_coord.1 - 1);
			if well.hit_test_rock(rock, rock_coord_below).is_air() {
				rock_coord = rock_coord_below;
			} else {
				well.paste_rock(rock, rock_coord);
				continue 'next_rock;
			}
		}
	}

	well.max_height() + 1
}

pub fn b(input: &str) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(17)).to_string(), "3068");
		assert_eq!(b(&test_input_as_string(17)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(17)).to_string(), "3217");
		assert_eq!(b(&input_as_string(17)).to_string(), "x");
	}
}
