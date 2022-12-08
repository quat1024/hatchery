use crate::*;

fn disjoint_find_2<const WINDOW_SIZE: usize>(input: &String) -> Option<usize> {
	let input = input.as_bytes();

	//iterate over the start of the window
	(0..input.len() - WINDOW_SIZE)
		//expand these out into full windows
		.map(|start| start..start + WINDOW_SIZE)
		//test each window to see if the uniqueness condition holds
		.find_map(|range| {
			let mut filter = [false; u8::MAX as usize];
			for b in &input[range.clone()] {
				if filter[*b as usize] {
					return None;
				}

				filter[*b as usize] = true;
			}

			Some(range.start + WINDOW_SIZE)
		})

	//TODO: probably possible to do this in a smooth motion, instead of restarting the search on every window-slide
}

fn run_a_on(input: String) -> impl Display {
	disjoint_find_2::<4>(&input).map(|x| x.to_string()).unwrap_or("not found".into())
}

fn run_b_on(input: String) -> impl Display {
	disjoint_find_2::<14>(&input).map(|x| x.to_string()).unwrap_or("not found".into())
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
