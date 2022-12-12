#![allow(dead_code, unused_variables)]
#![feature(iter_array_chunks)] //hehe
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)] //I like them.
#![allow(clippy::must_use_candidate)] //I don't really care about this.

//a "prelude" of sorts
pub use std::convert::Infallible;
pub use std::fmt::Display;
pub use std::ops::Range;
pub use std::str::FromStr;

pub use crate::tools::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

mod tools;

pub fn main() {
	println!("01 a {}", day01::a(&input_as_string(1)));
	println!("01 b {}", day01::b(&input_as_string(1)));
	println!("02 a {}", day02::a(&input_as_string(2)));
	println!("02 b {}", day02::b(&input_as_string(2)));
	println!("03 a {}", day03::a(&input_as_string(3)));
	println!("03 b {}", day03::b(&input_as_string(3)));
	println!("04 a {}", day04::a(&input_as_string(4)));
	println!("04 b {}", day04::b(&input_as_string(4)));
	println!("05 a {}", day05::a(&input_as_string(5)));
	println!("05 b {}", day05::b(&input_as_string(5)));
	println!("06 a {}", day06::a(&input_as_string(6)));
	println!("06 b {}", day06::b(&input_as_string(6)));
	println!("07 a {}", day07::a(&input_as_string(7)));
	println!("07 b {}", day07::b(&input_as_string(7)));
	println!("08 a {}", day08::a(&input_as_string(8)));
	println!("08 b {}", day08::b(&input_as_string(8)));
	println!("09 a {}", day09::a(&input_as_string(9)));
	println!("09 b {}", day09::b(&input_as_string(9)));
	println!("10 a {}", day10::a(&input_as_string(10)));
	println!("10 b \n{}", day10::b(&input_as_string(10))); //this one is ascii-art so give it a newline
	println!("11 a {}", day11::a(&input_as_string(11)));
	println!("11 b {}", day11::b(&input_as_string(11)));
	println!("12 a {}", day12::a(&input_as_string(12)));
	println!("12 b {}", day12::b(&input_as_string(12)));
}

// input handling //

/// Reads an input file in the `../input` directory to a string. Argument is the puzzle number.
/// This is likely the input as saved from the website.
#[must_use]
pub fn input_as_string(input_id: u8) -> String {
	gimme_input(&format!("{input_id:02}.txt"))
}

/// Reads a test input file in the `../input` directory to a string. Argument is the puzzle number.
/// A test input file is likely something given in the problem statement.
#[must_use]
pub fn test_input_as_string(input_id: u8) -> String {
	gimme_input(&format!("{input_id:02} small.txt"))
}

/// Reads a file in the `../input` directory to a string. Argument is the filename.
/// Attempts to course-correct if the pwd is set to a higher-up directory.
///
/// # Panics
///
/// Panics if the current pwd is unobtainable/broken, or if the file does not exist.
#[must_use]
pub fn gimme_input(input_name: &str) -> String {
	//clicking with the mouse on the "run" inlay above main() in vscode doesnt seem to set the pwd inside the cargo workspace
	//but `cargo run --bin` does, im pretty sure? or `cargo run -p`?
	let mut here = std::env::current_dir().unwrap();
	if !here.ends_with("advent2022") {
		here.push("advent2022");
	}
	std::fs::read_to_string([here, "input".into(), input_name.into()].iter().collect::<std::path::PathBuf>()).unwrap()
}
