use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;

use crate::*;

///consider these as column vectors; the grid
/// a1
/// b2
/// c3
///is represented as [[a, b, c], [1, 2, 3]]
#[derive(Clone, Debug)]
struct Grid<T>(Vec<Vec<T>>);

//boilerplate to access the grid with grid[x][y] instead of grid.0[x][y]
impl<T> Index<usize> for Grid<T> {
	type Output = Vec<T>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl<T> IndexMut<usize> for Grid<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

//ok
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

impl<T: Ord + Copy> Grid<T> {
	fn max(&self) -> T {
		//TODO: why .copied() is needed to compile?
		//ALso yeah it should return an option im just lazy. TODO fix that too
		self.0.iter().map(|column| column.iter().copied().max().unwrap()).max().unwrap()
	}
}

impl<T: ToString> Grid<T> {
	fn print2(&self) {
		//this process rotates it 90 degrees the other way i think, lol
		for column in &self.0 {
			for item in column {
				let x = item.to_string();
				print!(
					"{}",
					match x.len() {
						0 => " ",
						1 => &x,
						_ => ">",
					}
				);
			}
			println!()
		}
	}
}

impl Grid<u8> {
	fn parsey_parse(input: &str) -> Self {
		//the puzzle input is rotated 90 degrees when parsing it this way but it's not a big deal
		Grid(input.lines().map(|line| line.chars().map(|c| c.to_digit(10).expect("nondigit") as u8).collect()).collect())
	}

	fn print(&self) {
		//this process rotates it 90 degrees the other way i think, lol
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
				print!("{}", if *item { '#' } else { '.' })
			}
			println!()
		}
	}
}

fn compute_visibility_map(forest: &Grid<u8>) -> Grid<bool> {
	let mut result = Grid::new_same_size(forest, false);

	for x in 0..forest.width() {
		let mut tallest_tree_from_north = -1i16;
		for y in 0..forest.height() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_north {
				result.0[x][y] = true;
				tallest_tree_from_north = tree_here;
			}
		}

		let mut tallest_tree_from_south = -1i16;
		for y in (0..forest.height()).rev() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_south {
				result.0[x][y] = true;
				tallest_tree_from_south = tree_here;
			}
		}
	}

	for y in 0..forest.height() {
		let mut tallest_tree_from_west = -1i16;
		for x in 0..forest.width() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_west {
				result.0[x][y] = true;
				tallest_tree_from_west = tree_here;
			}
		}

		let mut tallest_tree_from_east = -1i16;
		for x in (0..forest.width()).rev() {
			let tree_here = forest[x][y] as i16;
			if tree_here > tallest_tree_from_east {
				result.0[x][y] = true;
				tallest_tree_from_east = tree_here;
			}
		}
	}

	result
}

fn compute_scenic_scores_map(forest: &Grid<u8>) -> Grid<usize> {
	let mut scenic_scores = Grid::new_same_size(forest, 0);

	fn count(forest: &Grid<u8>, house: u8, x: usize, y: usize, dx: isize, dy: isize) -> usize {
		let mut count = 0;
		for i in 1..isize::MAX {
			//painstakingly bounds check (extension trait might help here)
			if let Some(sample_x) = x.checked_add_signed(dx * i).filter(|hm| (0..forest.width()).contains(hm)) {
				if let Some(sample_y) = y.checked_add_signed(dy * i).filter(|hm| (0..forest.width()).contains(hm)) {
					//always count the tree, even if it terminates the iteration
					count += 1;
					
					if forest[sample_x][sample_y] >= house {
						break;
					}
					
				} else {
					break; //y
				}
			} else {
				break; //x
			}
		}

		count
	}

	for x in 0..forest.width() {
		for y in 0..forest.height() {
			//"To measure the viewing distance from a given tree, look up, down, left, and right from that tree;
			//stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration."
			let house = forest[x][y];

			//somewhat pretty, but doesn't work
			// let up_score    = (1..).take_while(|i| y.checked_sub(*i).is_some()).take_while(|step| forest[x][y - step] <= house).count();
			// let left_score  = (1..).take_while(|i| x.checked_sub(*i).is_some()).take_while(|step| forest[x - step][y] <= house).count();
			// let right_score = (1..).take_while(|i| x + i < forest.width()) .take_while(|step| forest[x + step][y] <= house).count();
			// let down_score  = (1..).take_while(|i| y + i < forest.height()).take_while(|step| forest[x][y + step] <= house).count();
			let up_score = count(forest, house, x, y, 0, -1);
			let right_score = count(forest, house, x, y, 1, 0);
			let down_score = count(forest, house, x, y, 0, 1);
			let left_score = count(forest, house, x, y, -1, 0);

			scenic_scores[x][y] = up_score * right_score * down_score * left_score
		}
	}

	scenic_scores
}

fn run_a_on(input: String) -> impl Display {
	compute_visibility_map(&Grid::parsey_parse(&input)).population().to_string()
}

fn run_b_on(input: String) -> impl Display {
	compute_scenic_scores_map(&Grid::parsey_parse(&input)).max().to_string()
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
