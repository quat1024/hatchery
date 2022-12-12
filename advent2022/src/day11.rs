use std::{io::Write, cell::RefCell, collections::VecDeque, borrow::Borrow};

use crate::*;

#[derive(Debug)]
struct Monkey {
	id: usize,
	items: RefCell<VecDeque<usize>>,
	op: Expr,
	test: usize, //always "divisible by __"
	result: (usize, usize),
	business: RefCell<usize>, //im strugglin bro
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

		Some(Self { id, items: RefCell::new(items), op, test, result, business: RefCell::new(0) })
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

pub fn a(input: String) -> impl Display {
	let mut simians: Vec<Monkey> = Vec::new();
	let mut lines = input.lines();
	while let Some(monke) = Monkey::parse(&mut lines) {
		simians.push(monke);
		lines.next(); //consume the blank line separating them
	}
	
	for round in 1..=20 {
		for monke in &simians {
			while let Some(item) = monke.items.borrow_mut().pop_front() {
				
				//apply the monke's operation
				let item = monke.op.run(item);
				
				let item = item / 3;
				
				//throw the item to another monke
				let who = if item % monke.test == 0 {
					monke.result.0
				} else {
					monke.result.1 
				};
				
				//it just so happens that no monkeys throw items to themself, so this is safe
				simians[who].items.borrow_mut().push_back(item);
				
				//feel accomplushed
				*monke.business.borrow_mut() += 1;
			}
		}
	}
	
	simians.sort_by_key(|m| m.business.borrow().clone());
	simians.reverse(); //Hi i lost like 15 minutes of debugging to this.
	
	let a = simians[0].business.borrow().clone();
	let b = simians[1].business.borrow().clone();
	(a * b).to_string()
}

pub fn b(input: String) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(test_input_as_string(11)).to_string(), "10605");
		//assert_eq!(b(test_input_as_string(11)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(a(input_as_string(11)).to_string(), "117624");
		//assert_eq!(b(input_as_string(11)).to_string(), "x");
	}

	#[test]
	fn test_number_from_soup() {
		assert_eq!(number_from_soup(&"Monkey 0:"), Some(0));
		assert_eq!(number_from_soup(&"12345 yeah"), Some(12345));
		assert_eq!(number_from_soup(&"If true: throw to monkey 2"), Some(2));
	}
}
