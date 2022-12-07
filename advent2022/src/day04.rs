use crate::*;

struct SectionAssignment {
	start: u16,
	end: u16,
}

impl FromStr for SectionAssignment {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split('-');

		//Least unidiomatic Rust code
		Ok(SectionAssignment { start: split.next().unwrap().parse().unwrap(), end: split.next().unwrap().parse().unwrap() })
	}
}

impl SectionAssignment {
	fn contains(&self, other: &Self) -> bool {
		return other.start >= self.start && other.end <= self.end;
	}

	fn overlaps(&self, other: &Self) -> bool {
		//It's late, and I'm tired
		for num in self.start..=self.end {
			if (other.start..=other.end).contains(&num) {
				return true;
			}
		}

		return false;
	}
}

fn run_a_on(input: String) -> impl Display {
	input
		.lines()
		.map(|line| {
			//Oh god its horrible
			let mut split = line.split(',');
			(SectionAssignment::from_str(split.next().unwrap()).unwrap(), SectionAssignment::from_str(split.next().unwrap()).unwrap())
		})
		.filter(|(left, right)| left.contains(&right) || right.contains(&left))
		.count()
}

fn run_b_on(input: String) -> impl Display {
	input
		.lines()
		.map(|line| {
			//Oh god its horrible
			let mut split = line.split(',');
			(SectionAssignment::from_str(split.next().unwrap()).unwrap(), SectionAssignment::from_str(split.next().unwrap()).unwrap())
		})
		.filter(|(left, right)| left.overlaps(&right))
		.count()
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(4))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(4))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(4)).to_string(), "2");
		assert_eq!(run_b_on(test_input_as_string(4)).to_string(), "4");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "580");
		assert_eq!(run_b().to_string(), "895");
	}
}
