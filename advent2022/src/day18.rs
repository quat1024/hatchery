use std::collections::HashMap;
use std::collections::HashSet;

use crate::*;

type Coord = (isize, isize, isize); //x, y, z

fn parse(input: &str) -> impl Iterator<Item = Coord> + '_ {
	input.lines().filter_map(|line| {
		let mut split = line.split(',');
		Some((split.next()?.parse().ok()?, split.next()?.parse().ok()?, split.next()?.parse().ok()?))
	})
}

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
	let coords: HashSet<_> = parse(input).collect();
	coords.iter().map(|coord| neighbors(coord).into_iter().filter(|n| !coords.contains(n)).count()).sum::<usize>()
}

pub fn b(input: &str) -> impl Display {
	#[derive(Copy, Clone, Hash, Eq, PartialEq)]
	enum Tile {
		Lava,
		Air,
	}

	//God this is a cute trick
	let mut map: HashMap<Coord, Tile> = parse(input).zip(std::iter::repeat(Tile::Lava)).collect();

	//i love to make six passes over the input for something that could be done in one! Im lazy
	let x_min = map.keys().map(|c| c.0).min().unwrap();
	let x_max = map.keys().map(|c| c.0).max().unwrap();
	let y_min = map.keys().map(|c| c.1).min().unwrap();
	let y_max = map.keys().map(|c| c.1).max().unwrap();
	let z_min = map.keys().map(|c| c.2).min().unwrap();
	let z_max = map.keys().map(|c| c.2).max().unwrap();
	let x_range = x_min..=x_max;
	let y_range = y_min..=y_max;
	let z_range = z_min..=z_max;
	let x_range_wide = (x_min - 1)..=(x_max + 1);
	let y_range_wide = (y_min - 1)..=(y_max + 1);
	let z_range_wide = (z_min - 1)..=(z_max + 1);

	//Flood-fill out the known air tiles
	let mut frontier: HashSet<Coord> = HashSet::new();
	frontier.insert((x_min, y_min, z_min)); //just assume this is outside lol
	while let Some(&point_of_interest) = frontier.iter().next() {
		frontier.remove(&point_of_interest);

		if let Some(Tile::Lava) = map.get(&point_of_interest) {
			panic!("frontier explored into the structure, weird")
		}

		map.insert(point_of_interest, Tile::Air);

		for next in neighbors(&point_of_interest)
			.into_iter()
			.filter(|c| x_range_wide.contains(&c.0) && y_range_wide.contains(&c.1) && z_range_wide.contains(&c.2) && !map.contains_key(c))
		{
			frontier.insert(next);
		}
	}

	//Assume all unknown places are lava i guess?
	for x in x_range {
		for y in y_range.clone() {
			for z in z_range.clone() {
				map.entry((x, y, z)).or_insert(Tile::Lava);
			}
		}
	}

	//Same solution as last time after a bit of fudging
	let coords: Vec<_> = map.iter().filter_map(|(&coord, &tile)| (tile == Tile::Lava).then_some(coord)).collect();
	coords.iter().map(|coord| neighbors(coord).into_iter().filter(|n| !coords.contains(n)).count()).sum::<usize>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(18)).to_string(), "64");
		assert_eq!(b(&test_input_as_string(18)).to_string(), "58");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(18)).to_string(), "4444");
		assert_ne!(b(&input_as_string(18)).to_string(), "2480"); //too tight of a bounding box around the flood-fill
		assert_eq!(b(&input_as_string(18)).to_string(), "2530");
	}
}
