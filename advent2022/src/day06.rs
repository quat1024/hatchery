use crate::*;

fn disjoint_find_2<const WINDOW_SIZE: usize>(input: &String) -> Option<usize> {
	let input = input.as_bytes();
	
	'next: for start in 0..input.len() - WINDOW_SIZE {
		let mut filter = [false; 255];
		for b in &input[start..start + WINDOW_SIZE] {
			if filter[(*b) as usize] {
				continue 'next;
			}
			
			filter[(*b) as usize] = true;
		}
		
		return Some(start + WINDOW_SIZE)
	}
	
	None
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
