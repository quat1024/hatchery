use crate::*;

fn disjoint_find_2<const WINDOW_SIZE: usize>(input: &str) -> Option<usize> {
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

pub fn a(input: &str) -> impl Display {
	disjoint_find_2::<4>(input).map(|x| x.to_string()).unwrap_or("not found".into())
}

pub fn b(input: &str) -> impl Display {
	disjoint_find_2::<14>(input).map(|x| x.to_string()).unwrap_or("not found".into())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(6)).to_string(), "7");
		assert_eq!(b(&test_input_as_string(6)).to_string(), "19");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(6)).to_string(), "1760");
		assert_eq!(b(&input_as_string(6)).to_string(), "2974");
	}
}
