use crate::*;

enum Insn {
	Noop,
	Addx(isize)
}

impl Insn {
	fn parse(line: &str) -> Self {
		if let Some(num) = line.strip_prefix("addx ") {
			Self::Addx(num.parse().expect("numeric"))
		} else {
			Self::Noop
		}
	}
}

pub fn a(input: String) -> impl Display {
	let insns = input.lines().map(Insn::parse).collect::<Vec<_>>();
	
	let mut x = 1;
	let mut cycle_count = 0;
	let mut answer = 0;
	
	let mut cycle_bump = |xx| {
		cycle_count += 1;
		if [20, 60, 100, 140, 180, 220].contains(&cycle_count)  {
			answer += xx * cycle_count;
		}
	};
	
	for insn in insns {
		match insn {
			Insn::Noop => {
				cycle_bump(x);
			},
			Insn::Addx(arg) => {
				cycle_bump(x);
				cycle_bump(x);
				x += arg;
			}
		}
	}
	
	answer
}

pub fn b(input: String) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(test_input_as_string(10)).to_string(), "13140");
		//assert_eq!(b(test_input_as_string(10)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(a(input_as_string(10)).to_string(), "12520");
		//assert_eq!(b(input_as_string(10)).to_string(), "x");
	}
}