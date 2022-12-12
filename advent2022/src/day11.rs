use std::cell::RefCell;
use std::collections::VecDeque;

use crate::*;

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
		//Assumptions I take about the input: Expressions are always binary operations.
		let mut bits = line.split_ascii_whitespace();
		let (l, c, r) = (bits.next()?, bits.next()?, bits.next()?);
		//If the argument is nonnumeric, the only other option is Expr::Old.
		let l = Box::new(l.parse::<usize>().map(Expr::Iconst).unwrap_or(Expr::Old));
		let r = Box::new(r.parse::<usize>().map(Expr::Iconst).unwrap_or(Expr::Old));
		//If the binop isn't multiplication, it's addition.
		Some(if c == "*" { Expr::Mul(l, r) } else { Expr::Add(l, r) })
	}
}

struct Monkey {
	id: usize,
	items: RefCell<VecDeque<usize>>,
	op: Expr,
	test: usize,            //always "divisible by __"
	result: (usize, usize), //true, false
}

impl Monkey {
	fn parse<'iterator, 'line>(lines: &'iterator mut impl Iterator<Item = &'line str>) -> Option<Self> {
		let id = number_from_soup(lines.next()?)?;
		let items = lines.next()?.trim_start_matches("  Starting items: ").split(", ").map(str::parse).collect::<Result<VecDeque<_>, _>>().ok()?;

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

fn do_it<const ROUNDS: usize, const DIVIDER: usize>(input: &str) -> usize {
	let mut simians: Vec<Monkey> = Vec::new();
	let mut lines = input.lines();
	while let Some(monke) = Monkey::parse(&mut lines) {
		simians.push(monke);
		lines.next(); //consume the blank line separating them in the input
	}

	//Required for part b, doesn't hurt part a.
	//In this case, the puzzle is small enough that the simple product of all the monkey's test values works too.
	//Using the least common multiple is just for style points.
	let modulus: usize = crate::tools::lcm_iter(simians.iter().map(|m| m.test));

	let mut business = vec![0; simians.len()];

	for round in 1..=ROUNDS {
		for monke in &simians {
			let mut items = monke.items.borrow_mut();
			business[monke.id] += items.len();
			while let Some(item) = items.pop_front() {
				let new_item = ((monke.op.run(item)) / DIVIDER) % modulus;
				let dest = &simians[monke.select_dest(new_item)];
				if std::ptr::eq(monke, dest) {
					//This case never comes up in the AoC input, but pathological cases (like the one constructed below) could panic.
					items.push_back(new_item);
				} else {
					//We need a RefCell because we currently have a borrow to one monkey, but want to mutate a different monkey.
					//The borrow to one monkey requires a borrow to the collection it comes from, which normally forbids
					//taking out mutable borrows to anything else in the collection.
					dest.items.borrow_mut().push_back(new_item);
				}
			}
		}
	}

	simians.sort_by_key(|m| business[m.id]);
	simians.reverse();
	business[simians[0].id] * business[simians[1].id]
}

pub fn a(input: &str) -> impl Display {
	do_it::<20, 3>(input).to_string()
}

pub fn b(input: &str) -> impl Display {
	do_it::<10000, 1>(input).to_string()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(11)).to_string(), "10605");
		assert_eq!(b(&test_input_as_string(11)).to_string(), "2713310158");
	}

	#[test]
	fn test_pathological() {
		//if a monkey throws an item to itself it'd cause a borrow_mut panic with my original solution
		//constructing this case requires being a bit careful that it doesn't infinite loop ;)
		assert_eq!(
			b("Monkey 0:
  Starting items: 1
  Operation: new = old + 1
  Test: divisible by 30
    If true: throw to monkey 1
    If false: throw to monkey 0
	
Monkey 1:
  Starting items: 2
  Operation: new = old + 2
  Test: divisible by 51
    If true: throw to monkey 0
    If false: throw to monkey 1")
			.to_string(),
			"399980000"
		);
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(11)).to_string(), "117624");
		assert_eq!(b(&input_as_string(11)).to_string(), "16792940265");
	}
}
