use crate::*;

///these are columns; the grid
/// a1x
/// b2y
/// c3z
///is represented as [[a, b, c], [1, 2, 3], [x, y, z]]
///uhh i think so anyway
struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
	fn width(&self) -> usize {
		self.len()
	}

	//assumes nonempty and nonragged
	fn height(&self) -> usize {
		self[0].len()
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

impl Grid<u8> {
	fn parse_forest(input: &str) -> Self {
		//the puzzle input is transposed when parsing it this way but it's not a big deal
		//TODO panics, doesn't check non-raggedness
		Grid(input.lines().map(|line| line.chars().map(|c| c.to_digit(10).expect("nondigit") as u8).collect()).collect())
	}
}

impl Grid<bool> {
	fn population(&self) -> usize {
		self.iter().map(|column| column.iter().filter(|x| **x).count()).sum()
	}
}

//and the following three impls are boilerplate to access the tuple struct in various ways without .0
//using a tuple struct instead of a type alias because i want to write the previous impls :V
impl<T> std::ops::Index<usize> for Grid<T> {
	type Output = Vec<T>;
	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index] //although here i use .0 to avoid infinite recursion :) probably a different way though
	}
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl<T> std::ops::Deref for Grid<T> {
	type Target = Vec<Vec<T>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn run_a_on(input: String) -> impl Display {
	let forest = &Grid::parse_forest(&input);
	let mut result = Grid::new_same_size(forest, false);

	for x in 0..forest.width() {
		let mut tallest_tree_from_north = -1i16;
		for y in 0..forest.height() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_north {
				result[x][y] = true;
				tallest_tree_from_north = tree_here;
			}
		}

		let mut tallest_tree_from_south = -1i16;
		for y in (0..forest.height()).rev() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_south {
				result[x][y] = true;
				tallest_tree_from_south = tree_here;
			}
		}
	}

	for y in 0..forest.height() {
		let mut tallest_tree_from_west = -1i16;
		for x in 0..forest.width() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_west {
				result[x][y] = true;
				tallest_tree_from_west = tree_here;
			}
		}

		let mut tallest_tree_from_east = -1i16;
		for x in (0..forest.width()).rev() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_east {
				result[x][y] = true;
				tallest_tree_from_east = tree_here;
			}
		}
	}

	result.population().to_string()
}

fn run_b_on(input: String) -> impl Display {
	trait UsizeExt {
		///basically this answers the question `bounds.contains(self + offset)`, but this trait wouldn't exist if that was easy to write in Rust
		///1. adding a usize to an isize is not allowed without careful handling
		///2. if `self` is 0 and `offset` is negative, the addition panics
		///3. i'd like to use an `if let` to handle the error cases instead of clumsy if-elsing everything
		///it's not terrible to write (it is a one-liner) but the edge-case handling is *noisy* and i'd like something less confusing
		fn offset_within(self, offset: isize, bounds: Range<usize>) -> Option<usize>;
	}

	impl UsizeExt for usize {
		fn offset_within(self, offset: isize, bounds: Range<usize>) -> Option<usize> {
			self.checked_add_signed(offset).filter(|i| bounds.contains(i))
		}
	}

	let forest = &Grid::parse_forest(&input);

	let mut best_scenic_score = 0usize;

	for x in 0..forest.width() {
		for y in 0..forest.height() {
			let house = forest[x][y];

			let count_fn = |dx: isize, dy: isize| -> usize {
				let mut count = 0;
				for i in 1..isize::MAX {
					if let (Some(sample_x), Some(sample_y)) = (x.offset_within(dx * i, 0..forest.width()), y.offset_within(dy * i, 0..forest.height())) {
						count += 1;

						if forest[sample_x][sample_y] >= house {
							break; //can't see past this tree from the treehouse
						}
					} else {
						break; //fell off the edge of the grid
					}
				}

				count
			};

			best_scenic_score = best_scenic_score.max(count_fn(0, -1) * count_fn(1, 0) * count_fn(0, 1) * count_fn(-1, 0));
		}
	}

	best_scenic_score.to_string()
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
		assert_eq!(run_b_on(test_input_as_string(8)).to_string(), "8");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "1688");
		assert_ne!(run_b().to_string(), "5752800"); //too high
		assert_eq!(run_b().to_string(), "410400");
	}
}
