#![allow(dead_code, unused_variables)]

use std::{error::Error, fs, path::PathBuf};

pub mod day01;
pub mod day02;

pub fn main() -> Result<(), Box<dyn Error>> {
	day02::run_b()
}

pub fn input_as_string(input_name: &str) -> String {
	//join the filename onto the "advent2022/input" directory
	fs::read_to_string(["advent2022", "input", input_name].iter().collect::<PathBuf>()).unwrap()
}