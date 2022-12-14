use std::iter::Peekable;

use crate::*;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Term {
	Iconst(usize),
	List(Vec<Term>),
}

impl Term {
	fn parse(line: &str) -> Option<Term> {
		Self::parse_rec(&mut line.chars().peekable())
	}

	fn parse_rec(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Term> {
		match chars.next() {
			Some(c) if c.is_ascii_digit() => {
				//Cheating by leveraging how the only two-digit number to appear in the input is 10
				if let Some('0') = chars.peek() {
					chars.next();
					Some(Term::Iconst(10))
				} else {
					Some(Term::Iconst((c as u8 - b'0') as usize))
				}
			},
			Some('[') => {
				let mut terms = Vec::new();
				if let Some(']') = chars.peek() {
					chars.next();
				} else {
					while let Some(term) = Self::parse_rec(chars) {
						terms.push(term);
						if let Some(']') = chars.next() {
							//also discards commas
							break;
						}
					}
				}
				Some(Term::List(terms))
			},
			_ => None,
		}
	}
}

impl Ord for Term {
	fn cmp(&self, right: &Self) -> std::cmp::Ordering {
		match (self, right) {
			(Self::Iconst(left_int), Self::Iconst(right_int)) => left_int.cmp(right_int),
			(Self::List(left_list), Self::List(right_list)) => left_list
				.iter()
				.zip(right_list.iter())
				.find_map(|(left_term, right_term)| {
					let result = left_term.cmp(right_term);
					result.is_ne().then_some(result)
				})
				.unwrap_or_else(|| left_list.len().cmp(&right_list.len())),
			(Self::Iconst(_), Self::List(_)) => Term::List(vec![self.clone()]).cmp(right),
			(Self::List(_), Self::Iconst(_)) => self.cmp(&Term::List(vec![right.clone()])),
		}
	}
}

impl PartialOrd for Term {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other)) //ordering is total
	}
}

pub fn a(input: &str) -> impl Display {
	crate::tools::chunks(input)
		.iter()
		.enumerate()
		.filter_map(|(index_minus_one, chunk)| match (Term::parse(chunk[0]), Term::parse(chunk[1])) {
			(Some(a), Some(b)) if a < b => Some(index_minus_one + 1),
			_ => None,
		})
		.sum::<usize>()
}

pub fn b(input: &str) -> impl Display {
	let mut terms: Vec<Term> = input.lines().filter_map(Term::parse).collect();

	let (d2, d6) = (Term::parse("[[2]]").unwrap(), Term::parse("[[6]]").unwrap());
	terms.push(d2.clone());
	terms.push(d6.clone());

	terms.sort(); //Ord :)
	(1 + terms.iter().enumerate().find(|x| *x.1 == d2).unwrap().0) * (1 + terms.iter().enumerate().find(|x| *x.1 == d6).unwrap().0)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(13)).to_string(), "13");
		assert_eq!(b(&test_input_as_string(13)).to_string(), "140");
	}

	#[test]
	fn real() {
		assert_ne!(a(&input_as_string(13)).to_string(), "5969"); //too low
		assert_eq!(a(&input_as_string(13)).to_string(), "6101");
		assert_eq!(b(&input_as_string(13)).to_string(), "21909");
	}
}

// impl Display for Term {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Term::Iconst(a) => a.fmt(f),
// 			Term::List(list) => {
// 				f.write_char('[')?;
// 				for (idx, term) in list.iter().enumerate() {
// 					term.fmt(f)?;
// 					if idx != list.len() - 1 {
// 						f.write_char(',')?;
// 					}
// 				}
// 				f.write_char(']')
// 			},
// 		}
// 	}
// }
