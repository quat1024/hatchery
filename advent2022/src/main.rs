#![allow(dead_code, unused_variables)]
#![feature(iter_array_chunks)] //hehe
#![feature(array_windows)] //hehe
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)] //I like them.
#![allow(clippy::must_use_candidate)] //I don't really care about this.

//a "prelude" of sorts
pub use std::convert::Infallible;
pub use std::fmt::Display;
pub use std::ops::Range;
pub use std::str::FromStr;
use std::time::Instant;

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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod tools;

pub fn main() {
	let now = Instant::now();
	do_it(|s| println!("{s}"));
	println!("Elapsed: {}ms", now.elapsed().as_millis());

	if std::env::args().any(|hm| hm == "long") {
		println!("Running them 100 more times!");
		let now = Instant::now();
		for _ in 0..100 {
			do_it(|s| {
				std::hint::black_box(s);
			});
		}
		println!("Elapsed: {}ms", now.elapsed().as_millis());
	}
}

fn do_it(mut output: impl FnMut(String)) {
	output(format!("01 a {}", day01::a(&input_as_string(1))));
	output(format!("01 b {}", day01::b(&input_as_string(1))));
	output(format!("02 a {}", day02::a(&input_as_string(2))));
	output(format!("02 b {}", day02::b(&input_as_string(2))));
	output(format!("03 a {}", day03::a(&input_as_string(3))));
	output(format!("03 b {}", day03::b(&input_as_string(3))));
	output(format!("04 a {}", day04::a(&input_as_string(4))));
	output(format!("04 b {}", day04::b(&input_as_string(4))));
	output(format!("05 a {}", day05::a(&input_as_string(5))));
	output(format!("05 b {}", day05::b(&input_as_string(5))));
	output(format!("06 a {}", day06::a(&input_as_string(6))));
	output(format!("06 b {}", day06::b(&input_as_string(6))));
	output(format!("07 a {}", day07::a(&input_as_string(7))));
	output(format!("07 b {}", day07::b(&input_as_string(7))));
	output(format!("08 a {}", day08::a(&input_as_string(8))));
	output(format!("08 b {}", day08::b(&input_as_string(8))));
	output(format!("09 a {}", day09::a(&input_as_string(9))));
	output(format!("09 b {}", day09::b(&input_as_string(9))));
	output(format!("10 a {}", day10::a(&input_as_string(10))));
	output(format!("10 b \n{}", day10::b(&input_as_string(10)))); //this one is ascii-art so give it a newline
	output(format!("11 a {}", day11::a(&input_as_string(11))));
	output(format!("11 b {}", day11::b(&input_as_string(11))));
	output(format!("12 a {}", day12::a(&input_as_string(12))));
	output(format!("12 b {}", day12::b(&input_as_string(12))));
	output(format!("13 a {}", day13::a(&input_as_string(13))));
	output(format!("13 b {}", day13::b(&input_as_string(13))));
	output(format!("14 a {}", day14::a(&input_as_string(14))));
	output(format!("14 b {}", day14::b(&input_as_string(14))));
	output(format!("15 a {}", day15::a(&input_as_string(15))));
	output(format!("15 b {}", day15::b(&input_as_string(15))));
	output(format!("16 a {}", day16::a(&input_as_string(16))));
	output(format!("16 b {}", day16::b(&input_as_string(16))));
	output(format!("17 a {}", day17::a(&input_as_string(17))));
	output(format!("17 b {}", day17::b(&input_as_string(17))));
	output(format!("18 a {}", day18::a(&input_as_string(18))));
	output(format!("18 b {}", day18::b(&input_as_string(18))));
	output(format!("19 a {}", day19::a(&input_as_string(19))));
	output(format!("19 b {}", day19::b(&input_as_string(19))));
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
