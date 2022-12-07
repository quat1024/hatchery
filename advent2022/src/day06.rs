use crate::*;

fn disjoint_find(input: &String, window_size: usize) -> Option<usize> {
	let mut set: std::collections::HashSet<char> = Default::default();

	'next: for start in 0..input.len() - window_size {
		set.clear();

		for c in input[start..start + window_size].chars() {
			if !set.insert(c) {
				continue 'next;
			}
		}

		return Some(start + window_size);
	}

	None
}

fn run_a_on(input: String) -> impl Display {
	disjoint_find(&input, 4).map(|x| x.to_string()).unwrap_or("not found".into())
}

fn run_b_on(input: String) -> impl Display {
	disjoint_find(&input, 14).map(|x| x.to_string()).unwrap_or("not found".into())
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
