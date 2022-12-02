use std::error::Error;

use crate::input_as_string;

pub fn run() -> Result<(), Box<dyn Error>> {
	let input = input_as_string("01.txt");
	
	//splitting on double blank lines is a cheap way to chop into sections
	let chunks = input.split("\n\n");
	
	//do the whole problem in a sexy unmaintainable oneliner
	let most_calories: u64 = chunks.into_iter()
		.map(|chunk| {
			chunk.lines()
				.map(|line| line.parse::<u64>())
				.filter_map(Result::ok)
				.sum::<u64>()
		})
		.max().unwrap_or_default();
	
	println!("most calories: {}", most_calories);
	
	Ok(())
}