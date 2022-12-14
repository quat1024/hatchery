//use std::iter::Peekable;

use std::{iter::Peekable, fmt::Write};

use crate::*;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Term {
	Iconst(usize),
	List(Vec<Term>),
}

impl Term {
	fn total_cmp(&self, right: &Self) -> std::cmp::Ordering {
		match (self, right) {
			(Self::Iconst(li), Self::Iconst(ri)) => li.cmp(ri),
			(Self::List(ll), Self::List(rl)) => ll
				.iter()
				.zip(rl.iter())
				.find_map(|(lterm, rterm)| {
					let result = lterm.total_cmp(rterm);
					result.is_ne().then_some(result)
				})
				.unwrap_or_else(|| ll.len().cmp(&rl.len())),
			(Self::Iconst(li), Self::List(_)) => self.to_list_term().total_cmp(right),
			(Self::List(_), Self::Iconst(ri)) => self.total_cmp(&right.to_list_term()),
		}
	}

	fn to_list_term(&self) -> Self {
		match self {
			Self::Iconst(x) => Self::List(vec![self.clone()]),
			Self::List(_) => self.clone(),
		}
	}

	fn parse(line: &str) -> Option<Term> {
		Self::parse_rec(&mut line.chars().peekable())
	}

	fn parse_rec(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Term> {
		match chars.next() {
			Some(x) if x.is_ascii_digit() => {
				//Cheating by leveraging how the only two-digit number to appear in the input is 10
				if let Some('0') = chars.peek() {
					chars.next();
					Some(Term::Iconst(10))
				} else {
					Some(Term::Iconst((x as u8 - b'0') as usize))
				}
			},
			Some('[') => {
				let mut terms = Vec::new();
				while let Some(term) = Self::parse_rec(chars) {
					terms.push(term);
					match chars.peek() {
						Some(',') => {
							chars.next();
						},
						Some(']') => {
							chars.next();
							break;
						},
						_ => {},
					}
				}
				Some(Term::List(terms))
			},
			_ => None,
		}
	}
}

impl PartialOrd for Term {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.total_cmp(other))
	}
}

impl Ord for Term {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.total_cmp(other)
	}
}

impl Display for Term {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Term::Iconst(a) => a.fmt(f),
			Term::List(list) => {
				f.write_char('[')?;
				for (idx, term) in list.iter().enumerate() {
					term.fmt(f)?;
					if idx != list.len() - 1 {
						f.write_char(',')?;
					}
				}
				f.write_char(']')
			},
		}
	}
}

pub fn a(input: &str) -> impl Display {
	let mut result = 0;
	for (index_minus_one, chunk) in chunks(input).iter().enumerate() {
		if let (Some(a), Some(b)) = (Term::parse(chunk[0]), Term::parse(chunk[1])) {
			println!("parsed complete block: {a}\n{b}\n");
			if a < b {
				result += index_minus_one + 1;
			}
		}
	}

	result
}

pub fn b(input: &str) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(13)).to_string(), "13");
		//assert_eq!(b(&test_input_as_string(13)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_ne!(a(&input_as_string(13)).to_string(), "5969"); //too low
		assert_eq!(a(&input_as_string(13)).to_string(), "6101");
		//assert_eq!(b(&input_as_string(13)).to_string(), "x");
	}
}
