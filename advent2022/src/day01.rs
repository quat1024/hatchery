use crate::*;

pub fn a(input: &str) -> impl Display {
	//do the whole problem in a sexy unmaintainable oneliner
	chunks(input).into_iter().map(|chunk| chunk.into_iter().map(str::parse::<usize>).filter_map(Result::ok).sum::<usize>()).max().unwrap()
}

pub fn b(input: &str) -> impl Display {
	//map into the amount of calories carried by one elf
	let mut calories: Vec<usize> =
		chunks(input).into_iter().map(|chunk| chunk.into_iter().map(str::parse::<usize>).filter_map(Result::ok).sum::<usize>()).collect();

	//sort and take the top three (lazy way to do it)
	calories.sort_unstable();
	calories.reverse();
	assert!(calories.len() >= 3, "at least three elves");
	calories[0] + calories[1] + calories[2]
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(1)).to_string(), "24000");
		assert_eq!(b(&test_input_as_string(1)).to_string(), "45000");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(1)).to_string(), "73211");
		assert_eq!(b(&input_as_string(1)).to_string(), "213958");
	}
}
