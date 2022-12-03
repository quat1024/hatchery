//#![allow(dead_code, unused_variables)]

use std::path::PathBuf;

pub mod day01;
pub mod day02;

pub fn main() {
	println!("01 a {}", day01::run_a());
	println!("01 b {}", day01::run_b());
	println!("02 a {}", day02::run_a());
	println!("02 b {}", day02::run_b());
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

	let path = [here, "input".into(), input_name.into()].iter().collect::<PathBuf>();

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
