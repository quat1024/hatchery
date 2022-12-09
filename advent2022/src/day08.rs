use crate::*;

///consider these as column vectors; the grid
/// a1
/// b2
/// c3
///is represented as [[a, b, c], [1, 2, 3]]
#[derive(Clone, Debug)]
struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
	fn width(&self) -> usize {
		self.0.len()
	}

	//assumes nonempty and nonragged
	fn height(&self) -> usize {
		self.0[0].len()
	}
}

impl<T: Clone> Grid<T> {
	fn new(width: usize, height: usize, thing: T) -> Self {
		Grid(std::iter::repeat(std::iter::repeat(thing).take(height).collect()).take(width).collect())
	}

	fn new_same_size<X>(other: &Grid<X>, thing: T) -> Self {
		Self::new(other.width(), other.height(), thing)
	}
}

// impl<T: Default> Grid<T> {
// 	fn clear(&mut self) {
// 		for column in self.0.iter_mut() {
// 			for item in column {
// 				*item = T::default()
// 			}
// 		}
// 	}
// }

impl Grid<u8> {
	fn parsey_parse(input: &str) -> Self {
		//the puzzle input is rotated 90 degrees when parsing it this way but it's not a big deal
		Grid(input.lines().map(|line| line.chars().map(|c| c.to_digit(10).expect("nondigit") as u8).collect()).collect())
	}

	fn print(&self) {
		//this probably rotates it 90 degrees again
		for column in &self.0 {
			for item in column {
				print!("{}", item.to_string())
			}
			println!()
		}
	}
}

impl Grid<bool> {
	fn population(&self) -> usize {
		self.0.iter().map(|column| column.iter().filter(|x| **x).count()).sum()
	}

	fn print(&self) {
		//rotates it 90 degrees
		for column in &self.0 {
			for item in column {
				print!("{}", if *item { '#' } else { ' ' })
			}
			println!()
		}
	}
}

fn compute_visibility_map(forest: &Grid<u8>) -> Grid<bool> {
	let mut north_visibility_map = Grid::new_same_size(forest, false);
	let mut east_visibility_map = Grid::new_same_size(forest, false);
	let mut south_visibility_map = Grid::new_same_size(forest, false);
	let mut west_visibility_map = Grid::new_same_size(forest, false);

	for x in 0..forest.width() {
		let mut tallest_tree_from_north = -1i16;
		for y in 0..forest.height() {
			let tree_here = forest.0[x][y] as i16;
			if tree_here > tallest_tree_from_north {
				north_visibility_map.0[x][y] = true;
				tallest_tree_from_north = tree_here;
			}
		}

		let mut tallest_tree_from_south = -1i16;
		for y in (0..forest.height()).rev() {
			let tree_here = forest.0[x][y] as i16;
			if tree_here > tallest_tree_from_south {
				south_visibility_map.0[x][y] = true;
				tallest_tree_from_south = tree_here;
			}
		}
	}

	//east, casting rays left (-x)
	for y in 0..forest.height() {
		let mut tallest_tree_from_west = -1i16;
		for x in 0..forest.width() {
			let tree_here = forest.0[x][y] as i16;
			if tree_here > tallest_tree_from_west {
				west_visibility_map.0[x][y] = true;
				tallest_tree_from_west = tree_here;
			}
		}

		let mut tallest_tree_from_east = -1i16;
		for x in (0..forest.width()).rev() {
			let tree_here = forest.0[x][y] as i16;
			if tree_here > tallest_tree_from_east {
				east_visibility_map.0[x][y] = true;
				tallest_tree_from_east = tree_here;
			}
		}
	}

	//finally zip them together
	let mut result = Grid::new_same_size(forest, false);
	for x in 0..result.width() {
		for y in 0..result.height() {
			if north_visibility_map.0[x][y] || east_visibility_map.0[x][y] || south_visibility_map.0[x][y] || west_visibility_map.0[x][y] {
				result.0[x][y] = true;
			}
		}
	}

	result
}

fn run_a_on(input: String) -> impl Display {
	let forest = Grid::parsey_parse(&input);
	forest.print();
	let visibility_map = compute_visibility_map(&forest);
	visibility_map.print();
	visibility_map.population().to_string()
}

fn run_b_on(input: String) -> impl Display {
	"x"
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(8))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(8))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(8)).to_string(), "21");
		//assert_eq!(run_b_on(test_input_as_string(8)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "1688");
		//assert_eq!(run_b().to_string(), "x");
	}
}
