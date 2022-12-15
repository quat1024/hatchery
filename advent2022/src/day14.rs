use std::fmt::Write;

use crate::*;

type Coord = (isize, isize); //(row, col)

fn parse_coord(x: &str) -> Option<Coord> {
	let mut split = x.trim().split(',');
	let col = split.next()?.parse::<isize>().ok()?;
	let row = split.next()?.parse::<isize>().ok()?;
	Some((row, col))
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
	Air,
	Wall,
	Sand,
}

impl Tile {
	fn passable(self) -> bool {
		self == Tile::Air
	}
}

impl From<Tile> for char {
	fn from(val: Tile) -> Self {
		match val {
			Tile::Air => '.',
			Tile::Wall => '#',
			Tile::Sand => 'o',
		}
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(<Tile as Into<char>>::into(*self)) //HUHH
	}
}

struct Map {
	map: Vec<Vec<Tile>>,
}

impl Map {
	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::manual_map)] //Im gonna change it
	fn get(&self, coord: Coord) -> Option<Tile> {
		if let Some(row) = self.map.get(coord.0 as usize) {
			//TODO not bad
			Some(row.get(coord.1 as usize).copied().unwrap_or(Tile::Air))
		} else {
			None
		}
	}

	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::cast_possible_wrap)]
	fn set(&mut self, coord: Coord, tile: Tile) {
		//TODO not badbad bad
		if (self.map[coord.0 as usize].len() as isize) <= coord.1 {
			self.map[coord.0 as usize].extend(std::iter::repeat(Tile::Air).take(40)); //i dont wanna Calculate it my brain fried
		}
		//If you fall off of the left side of the grid... Ummm uh.. skill issue

		self.map[coord.0 as usize][coord.1 as usize] = tile;
	}

	#[allow(clippy::cast_sign_loss)]
	fn parse(input: &str, part2funny: bool) -> Map {
		let tubes: Vec<Vec<Coord>> = input.lines().map(|tube| tube.split(" -> ").filter_map(parse_coord).collect()).collect();

		let max_row = tubes.iter().flatten().map(|c| c.0).max().unwrap() as usize;
		let max_col = tubes.iter().flatten().map(|c| c.1).max().unwrap() as usize;

		let blank_row = vec![Tile::Air; max_col + 1];
		let mut map = Map { map: std::iter::repeat(blank_row).take(max_row + 10).collect() }; //10 for part2funny buffer

		for tube in tubes {
			for &[start, end] in tube.array_windows() {
				let drow = (end.0 - start.0).signum();
				let dcol = (end.1 - start.1).signum();
				let mut cursor = start;
				while cursor != end {
					map.set(cursor, Tile::Wall);
					cursor.0 += drow;
					cursor.1 += dcol;
				}
				map.set(cursor, Tile::Wall); //THANK YOU JULIAN I LOST LIKE 1858 YEARS DEBUGGING THIS
			}
		}

		if part2funny {
			for col in 0..max_row + 1000 {
				//PROBABLY GOOD ENOUGH
				map.set(((max_row + 2).try_into().unwrap(), col.try_into().unwrap()), Tile::Wall);
			}
		}

		map
	}

	fn drop_sand(&mut self) -> SandDropResult {
		let mut cursor = (0, 500);

		if let Some(t) = self.get(cursor) {
			if !t.passable() {
				return SandDropResult::Blocked;
			}
		}

		loop {
			let below = (cursor.0 + 1, cursor.1);

			if let Some(below_tile) = self.get(below) {
				if below_tile.passable() {
					cursor = below;
					continue;
				}

				let left = (below.0, below.1 - 1);
				let left_tile = self.get(left).unwrap();

				if left_tile.passable() {
					cursor = left;
					continue;
				}

				let right = (below.0, below.1 + 1);
				let right_tile = self.get(right).unwrap();
				if right_tile.passable() {
					cursor = right;
					continue;
				}

				self.set(cursor, Tile::Sand);
				return SandDropResult::Landed;
			}

			println!("fall out at {cursor:?}");
			return SandDropResult::FellOut;
		}
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.map {
			for item in row {
				let c: char = <Tile as Into<char>>::into(*item);
				f.write_char(c)?;
				f.write_char(',')?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SandDropResult {
	Landed,
	FellOut,
	Blocked,
}

pub fn a(input: &str) -> impl Display {
	let mut map = Map::parse(input, false);
	let mut grain_count = 0;
	while map.drop_sand() == SandDropResult::Landed {
		grain_count += 1;
	}
	grain_count
}

pub fn b(input: &str) -> impl Display {
	let mut map = Map::parse(input, true);
	let mut grain_count = 0;
	while map.drop_sand() != SandDropResult::Blocked {
		grain_count += 1;
	}
	grain_count
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(14)).to_string(), "24");
		assert_eq!(b(&test_input_as_string(14)).to_string(), "93");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(14)).to_string(), "592");
		assert_eq!(b(&input_as_string(14)).to_string(), "30367");
	}
}
