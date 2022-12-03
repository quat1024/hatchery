use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

use crate::*;

struct Sack {
	left: String,
	right: String,
}

impl FromStr for Sack {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (left, right) = s.split_at(s.len() / 2);
		
		assert!(left.len() == right.len(), "same number of items in both sacks");
		
		Ok(Sack {
			left: left.to_owned(),
			right: right.to_owned()
		})
	}
}

impl Sack {
	fn find_same_item(&self) -> char {
		for item in self.left.chars() {
			if self.right.contains(item) {
				return item;
			}
		}
		
		panic!("couldn't find shared item")
	}
}

fn priority(item: char) -> u16 {
	match item {
		'a'..='z' => item as u16 - 'a' as u16 + 1,
		'A'..='Z' => item as u16 - 'A' as u16 + 27,
		_ => panic!("unexpected item in bagging area")
	}
}

fn run_a_on(input: String) -> impl Display {
	let sacks: Result<Vec<Sack>, _> = input.lines().map(Sack::from_str).collect();
	let sacks = sacks.unwrap();
	
	sacks.iter().map(Sack::find_same_item).map(priority).sum::<u16>()
}

fn run_b_on(input: String) -> impl Display {
	"x"
}

pub fn run_a() -> impl Display {
	//run_a_on(input_as_string(3))
	run_a_on(input_as_string(3))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(3))
}

mod test {
	use super::*;

	#[test]
	fn test_priority() {
		assert_eq!(priority('p'), 16);
		assert_eq!(priority('L'), 38);
	}
	
	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(3)).to_string(), "157");
		//assert_eq!(run_b_on(test_input_as_string(3)).to_string(), "70");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "7811");
		//assert_eq!(run_b().to_string(), "x");
	}
}
