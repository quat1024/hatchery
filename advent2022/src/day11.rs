use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Write;

use crate::*;

#[derive(Debug)]
struct Monkey {
	id: usize,
	items: RefCell<VecDeque<usize>>,
	op: Expr,
	test: usize, //always "divisible by __"
	result: (usize, usize),
}

#[derive(Clone, Debug)]
enum Expr {
	Old,
	Iconst(usize),
	Add(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
	fn run(&self, old: usize) -> usize {
		match self {
			Expr::Old => old,
			Expr::Iconst(i) => *i,
			Expr::Add(l, r) => l.run(old) + r.run(old),
			Expr::Mul(l, r) => l.run(old) * r.run(old),
		}
	}

	fn parse(line: &str) -> Option<Self> {
		//Assuming things about the input:
		//They're always binary operations.
		let mut bits = line.split_ascii_whitespace();
		let (l, c, r) = (bits.next()?, bits.next()?, bits.next()?);
		//If the argument is nonnumeric, it's always Expr::Old.
		let l = Box::new(l.parse::<usize>().map(Expr::Iconst).unwrap_or(Expr::Old));
		let r = Box::new(r.parse::<usize>().map(Expr::Iconst).unwrap_or(Expr::Old));
		//If the binop isn't multiplication, it's addition.
		Some(if c == "*" { Expr::Mul(l, r) } else { Expr::Add(l, r) })
	}
}

impl Monkey {
	fn parse<'a, 'b, L>(lines: &'a mut L) -> Option<Self>
	where
		L: Iterator<Item = &'b str>,
	{
		let id = number_from_soup(lines.next()?)?;
		let items = lines.next()?.trim_start_matches("  Starting items: ").split(", ").map(|x| x.parse::<usize>()).collect::<Result<VecDeque<_>, _>>().ok()?;

		let op = Expr::parse(lines.next()?.trim_start_matches("  Operation: new = "))?;

		let test = number_from_soup(lines.next()?)?;
		let result = (number_from_soup(lines.next()?)?, number_from_soup(lines.next()?)?);

		Some(Self { id, items: RefCell::new(items), op, test, result })
	}

	fn select_dest(&self, item: usize) -> usize {
		if item % self.test == 0 {
			self.result.0
		} else {
			self.result.1
		}
	}
}

fn number_from_soup(line: &str) -> Option<usize> {
	//my shitty ass algorithm doesnt work when the line ends with a number
	let mut line = line.to_string();
	line.push('.');

	if let Some((start, _)) = line.chars().enumerate().find(|c| c.1.is_ascii_digit()) {
		if let Some((len, _)) = line.chars().skip(start + 1).enumerate().find(|c| !c.1.is_ascii_digit()) {
			return line[start..start + len + 1].to_string().parse().ok();
		}
	}

	None
}

fn do_it<const ROUNDS: usize, const DIVIDER: usize>(input: &str) -> usize {
	let mut simians: Vec<Monkey> = Vec::new();
	let mut lines = input.lines();
	while let Some(monke) = Monkey::parse(&mut lines) {
		simians.push(monke);
		lines.next(); //consume the blank line separating them in the input
	}

	let modulus: usize = simians.iter().map(|m| m.test).product();
	let mut business = vec![0; simians.len()];

	for round in 1..=ROUNDS {
		for monke in &simians {
			let mut items = monke.items.borrow_mut();
			business[monke.id] += items.len();
			while let Some(item) = items.pop_front() {
				let new_item = ((monke.op.run(item)) / DIVIDER) % modulus;
				//it just so happens that no monkeys throw items to themself, so this borrow_mut won't kaboom
				simians[monke.select_dest(new_item)].items.borrow_mut().push_back(new_item);
			}
		}
	}

	simians.sort_by_key(|m| business[m.id]);
	simians.reverse();

	let a = business[simians[0].id];
	let b = business[simians[1].id];
	a * b
}

pub fn a(input: String) -> impl Display {
	do_it::<20, 3>(&input).to_string()
}

pub fn b(input: String) -> impl Display {
	do_it::<10000, 1>(&input).to_string()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(test_input_as_string(11)).to_string(), "10605");
		assert_eq!(b(test_input_as_string(11)).to_string(), "2713310158");
	}

	#[test]
	fn real() {
		assert_eq!(a(input_as_string(11)).to_string(), "117624");
		assert_eq!(b(input_as_string(11)).to_string(), "16792940265");
	}

	#[test]
	fn test_number_from_soup() {
		assert_eq!(number_from_soup(&"Monkey 0:"), Some(0));
		assert_eq!(number_from_soup(&"12345 yeah"), Some(12345));
		assert_eq!(number_from_soup(&"If true: throw to monkey 2"), Some(2));
	}
}
