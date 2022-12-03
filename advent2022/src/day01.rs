use std::fmt::Display;

use crate::*;

fn run_a_on(input: String) -> impl Display {
	//do the whole problem in a sexy unmaintainable oneliner
	chunks(&input).into_iter().map(|chunk| chunk.into_iter().map(|line| line.parse::<u64>()).filter_map(Result::ok).sum::<u64>()).max().unwrap()
}

fn run_b_on(input: String) -> impl Display {
	//map into the amount of calories carried by one elf
	let mut calories: Vec<u64> =
		chunks(&input).into_iter().map(|chunk| chunk.into_iter().map(|line| line.parse::<u64>()).filter_map(Result::ok).sum::<u64>()).collect();

	//sort and take the top three (lazy way to do it)
	calories.sort();
	calories.reverse();
	assert!(calories.len() >= 3, "at least three elves");
	calories[0] + calories[1] + calories[2]
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(1))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(1))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(1)).to_string(), "24000");
		assert_eq!(run_b_on(test_input_as_string(1)).to_string(), "45000");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "73211");
		assert_eq!(run_b().to_string(), "213958");
	}
}
