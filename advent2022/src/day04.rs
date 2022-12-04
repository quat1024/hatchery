use std::fmt::Display;

use crate::*;

fn run_a_on(input: String) -> impl Display {
	"x"
}

fn run_b_on(input: String) -> impl Display {
	"x"
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(3))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(3))
}

mod test {
	//use super::*;
	
	#[test]
	fn test() {
		//assert_eq!(run_a_on(test_input_as_string(3)).to_string(), "157");
		//assert_eq!(run_b_on(test_input_as_string(3)).to_string(), "70");
	}

	#[test]
	fn real() {
		//assert_eq!(run_a().to_string(), "7811");
		//assert_eq!(run_b().to_string(), "2639");
	}
}
