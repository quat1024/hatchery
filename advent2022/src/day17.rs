use std::collections::HashMap;

use crate::*;

type Coord = (isize, isize); //column, row

type HeightProfile = [isize; 7];

#[derive(Copy, Clone, Debug)]
enum RockShape {
	Flat,
	Plus,
	BackwardL,
	I,
	O,
}

impl RockShape {
	fn all() -> Vec<RockShape> {
		[RockShape::Flat, RockShape::Plus, RockShape::BackwardL, RockShape::I, RockShape::O].into()
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
		self.columns.iter().map(Self::max_column_height).max().unwrap_or(-1)
	}

	fn max_column_height(col: &HashMap<isize, bool>) -> isize {
		*col.keys().max().unwrap_or(&-1isize)
	}

	fn answer(&self) -> isize {
		self.max_height() + 1
	}

	fn hit_something(&self, coord: Coord) -> bool {
		!(0..=6).contains(&coord.0) || coord.1 < 0 || self.get_column(coord.0).contains_key(&coord.1)
	}

	fn rock_hit_something(&self, shape: RockShape, coord: Coord) -> bool {
		for rock_cell in shape.cells() {
			if self.hit_something((coord.0 + rock_cell.0, coord.1 + rock_cell.1)) {
				return true;
			}
		}
		false
	}

	fn paste_rock(&mut self, shape: RockShape, coord: Coord) {
		for rock_cell in shape.cells() {
			self.get_column_mut(coord.0 + rock_cell.0).insert(coord.1 + rock_cell.1, true);
		}
	}

	#[allow(clippy::cast_possible_wrap)]
	fn height_profile(&self) -> HeightProfile {
		let max_height = self.max_height();

		let mut result = [0isize; 7];
		for (idx, result) in result.iter_mut().enumerate() {
			*result = max_height - Self::max_column_height(self.get_column(idx as isize));
		}

		result
	}

	fn drop_one<T>(&mut self, gusts: &mut impl Iterator<Item = (T, char)>, rock: RockShape) {
		let mut rock_coord = (2, self.max_height() + 4);

		loop {
			let blow_offset = match gusts.next() {
				Some((_, '<')) => -1,
				Some((_, '>')) => 1,
				_ => panic!("unexpected item in bagging area"),
			};

			let blown_rock_coord = (rock_coord.0 + blow_offset, rock_coord.1);
			if !self.rock_hit_something(rock, blown_rock_coord) {
				rock_coord = (blown_rock_coord.0, blown_rock_coord.1);
			}

			let rock_coord_below = (rock_coord.0, rock_coord.1 - 1);
			if self.rock_hit_something(rock, rock_coord_below) {
				self.paste_rock(rock, rock_coord);
				break;
			}
			
			rock_coord = rock_coord_below;
		}
	}
}

///Hope you're on 64 bit! Yeah i should use explicity sized types huh
#[allow(clippy::cast_possible_wrap)]
fn drop_it_like_its_hot<const LIMIT: usize>(input: &str) -> isize {
	#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Debug)]
	struct StateKey {
		gust_index: usize,
		rock_index: usize,
		height_profile: [isize; 7],
	}

	#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Debug)]
	struct StateValue {
		rock_index: usize,
		well_height: isize,
	}

	//putting enumerate() before cycle() so that the index is *within* the cycle
	let mut gusts = input.lines().next().unwrap().trim().chars().enumerate().cycle().peekable();
	let rocks_dont_drop_pls = RockShape::all(); //i hate rust
	let mut rocks = rocks_dont_drop_pls.iter().copied().enumerate().cycle().peekable();
	let mut well = Well::default();

	let mut cache = HashMap::<StateKey, StateValue>::new();

	let mut rock_index = 0;
	while rock_index < LIMIT {
		rock_index += 1;

		well.drop_one(&mut gusts, rocks.next().unwrap().1);

		let skey = StateKey { gust_index: gusts.peek().unwrap().0, rock_index: rocks.peek().unwrap().0, height_profile: well.height_profile() };
		let svalue = StateValue { rock_index, well_height: well.max_height() };
		if let Some(last_value) = cache.insert(skey, svalue) {
			let cycle_length = rock_index - last_value.rock_index;
			let cycles_to_go = (LIMIT - rock_index) / cycle_length; //flooring division

			//how many rocks to fast-forward
			rock_index += cycles_to_go * cycle_length;
			//how many tiles of height we fast forwarded though
			let bonus = (cycles_to_go as isize) * (svalue.well_height - last_value.well_height);

			//finish up the last cycle
			for _ in rock_index..LIMIT {
				well.drop_one(&mut gusts, rocks.next().unwrap().1);
			}
			
			return well.answer() + bonus;
		}
	}

	//i guess there wasn't a cycle. fancy that
	well.answer()
}

pub fn a(input: &str) -> impl Display {
	drop_it_like_its_hot::<2022>(input)
}

pub fn b(input: &str) -> impl Display {
	drop_it_like_its_hot::<1_000_000_000_000>(input)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(17)).to_string(), "3068");
		assert_eq!(b(&test_input_as_string(17)).to_string(), "1514285714288");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(17)).to_string(), "3217");
		assert_eq!(b(&input_as_string(17)).to_string(), "1585673352422");
	}
}
