#![allow(dead_code, unused_variables)]
#![allow(unused_imports)] //rurun_a_onsinput_as_string(3)eems to be buggy about use super::* in tests?
#![feature(iter_array_chunks)] //hehe

//a "prelude" of sorts
pub use std::convert::Infallible;
pub use std::fmt::Display;
pub use std::ops::Range;
pub use std::str::FromStr;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn main() {
	println!("01 a {}", day01::a(input_as_string(1)));
	println!("01 b {}", day01::b(input_as_string(1)));
	println!("02 a {}", day02::a(input_as_string(2)));
	println!("02 b {}", day02::b(input_as_string(2)));
	println!("03 a {}", day03::a(input_as_string(3)));
	println!("03 b {}", day03::b(input_as_string(3)));
	println!("04 a {}", day04::a(input_as_string(4)));
	println!("04 b {}", day04::b(input_as_string(4)));
	println!("05 a {}", day05::a(input_as_string(5)));
	println!("05 b {}", day05::b(input_as_string(5)));
	println!("06 a {}", day06::a(input_as_string(6)));
	println!("06 b {}", day06::b(input_as_string(6)));
	println!("07 a {}", day07::a(input_as_string(7)));
	println!("07 b {}", day07::b(input_as_string(7)));
	println!("08 a {}", day08::a(input_as_string(8)));
	println!("08 b {}", day08::b(input_as_string(8)));
}

// input handling //

pub fn input_as_string(input_id: u8) -> String {
	gimme_input(&format!("{:02}.txt", input_id))
}

pub fn test_input_as_string(input_id: u8) -> String {
	gimme_input(&format!("{:02} small.txt", input_id))
}

pub fn gimme_input(input_name: &str) -> String {
	//todo the path stuff is really annoying, clicking with the mouse on the "run" button in vscode doesn't seem to set the pwd?

	let mut here = std::env::current_dir().unwrap();
	if !here.ends_with("advent2022") {
		here.push("advent2022")
	}

	let path = [here, "input".into(), input_name.into()].iter().collect::<std::path::PathBuf>();

	std::fs::read_to_string(path).unwrap()
}

// util ! //

/// Sometimes in AoC you get input that's a "list of lists", where two blank lines separate each list.
/// Splitting on \n\n works, usually, unless you're on Windows and get carriage returns too! This can happen if you use the clipboard.
///
/// So this function splits into chunks the "hard way". It scans a string line-by-line, copying a slice to each line into a bucket.
/// When a blank line is encountered, the bucket is added to the list-of-lists.
pub fn chunks<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
	let mut chunks: Vec<Vec<&'a str>> = Vec::new();
	let mut bucket: Vec<&'a str> = Vec::new();

	for line in input.lines() {
		if line.trim().is_empty() {
			//finding an empty string to delimit each bucket
			//bucket.is_empty may be true if there's two blank lines in a row
			if !bucket.is_empty() {
				//stash away the current bucket, and make `bucket` point to a newly allocated one, in one step
				chunks.push(std::mem::take(&mut bucket));
			}
		} else {
			bucket.push(&line);
		}
	}

	//the last one
	if !bucket.is_empty() {
		chunks.push(bucket);
	}

	chunks
}

#[cfg(test)]
mod main {
	use super::*;

	#[test]
	fn test_chunks() {
		assert_eq!(
			chunks(
				&"part1
part1
part1

part2
part2

part3"
			),
			vec![vec!["part1", "part1", "part1"], vec!["part2", "part2"], vec!["part3"],]
		);
	}
}
