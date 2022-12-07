use crate::*;

fn run_a_on(input: String) -> impl Display {
	let mut idx: usize = 3; //start off-by-three because the window size
	for &[a, b, c, d] in input.chars().collect::<Vec<_>>().array_windows() {
		idx += 1;
		if a != b && a != c && a != d && b != c && b != d && c != d {
			return idx.to_string();
		}
	}

	"not found".into()
}

fn run_b_on(input: String) -> impl Display {
	//goddammit no cute tricks this time LOL

	//random-access string
	let input = input.chars().collect::<Vec<_>>();
	let window_size = 14;

	let mut set: std::collections::HashSet<char> = Default::default();

	for start in 0..input.len() - window_size {
		set.clear();
		for c_idx in start..start + window_size {
			set.insert(input[c_idx]);
		}

		if set.len() == window_size {
			return (start + window_size).to_string();
		}
	}

	"not found".into()
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(6))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(6))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(6)).to_string(), "7");
		assert_eq!(run_b_on(test_input_as_string(6)).to_string(), "19");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "1760");
		assert_eq!(run_b().to_string(), "2974");
	}
}
