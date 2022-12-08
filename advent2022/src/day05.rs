use crate::*;

struct Shipyard {
	///zero-indexed (unlike the puzzle input)
	///stacks grow upwards, the bottom of each pile is element 0
	stacks: Vec<Vec<char>>,
}

struct Instruction {
	count: usize,
	src: usize,
	dst: usize,
}

impl Shipyard {
	fn new(pile_count: usize) -> Shipyard {
		Shipyard { stacks: std::iter::repeat(Vec::new()).take(pile_count).collect() }
	}

	fn answer(&self) -> String {
		self.stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).collect()
	}

	fn run_instruction(&mut self, insn: &Instruction) {
		let src = insn.src;
		let dst = insn.dst;

		for _ in 0..insn.count {
			if let Some(crate_label) = self.stacks[src].pop() {
				self.stacks[dst].push(crate_label);
			} else {
				panic!("pop from empty stack? state rn is {}", self.answer())
			}
		}
	}

	fn run_instruction_9001(&mut self, insn: &Instruction) {
		let src = &mut self.stacks[insn.src];

		//you cant borrow both at once for some stupid reason so i will have to collect into a structure first
		let mut moving_bits = Vec::<char>::new();
		for what in &src[src.len() - insn.count..] {
			moving_bits.push(*what);
		}

		src.truncate(src.len() - insn.count);
		self.stacks[insn.dst].append(&mut moving_bits);
	}

	//TODO not Infallible result
	///this function here consumes the iterator only until it's parsed the whole shipyard
	fn from_lines_iterator<'iter, 'lines, X>(lines: &'iter mut X) -> Result<Shipyard, Infallible>
	where
		X: Iterator<Item = &'lines str>,
	{
		//(step 1) remove all the whitespace and the 1 2 3 footer and stuff
		let mut lines_cleaned = Vec::<Vec<char>>::new();

		'done: for line in lines {
			let mut cleaned_line = String::new();
			let mut cs = line.chars().fuse();

			loop {
				//grab chars four at a time
				let (a, b, c, _) = (cs.next(), cs.next(), cs.next(), cs.next());

				//the end of the string?
				if a.is_none() {
					break;
				}

				//that 1 2 3 footer line?
				if let Some(bchar) = b {
					if bchar.is_ascii_digit() {
						break 'done;
					}
				}

				//a crate?
				if let (Some('['), Some(crate_label), Some(']')) = (a, b, c) {
					cleaned_line.push(crate_label);
				}

				//air?
				if let Some(' ') = b {
					cleaned_line.push(' ');
				}
			}

			lines_cleaned.push(cleaned_line.chars().collect()); //String -> Vec<char>, easier to work with
		}

		//(step 2) transpose the matrix!
		let mut stacks = Vec::new();
		for stack_id in 0..lines_cleaned.last().expect("at least one nonempty pile").len() {
			//the first element of the stack is taken from the last line of lines_cleaned
			//the second element is taken from the line before that, and so on
			//until reaching the top of the matrix
			let mut stack = Vec::with_capacity(stacks.len());

			'next: for line_no in (0..lines_cleaned.len()).rev() {
				let c = lines_cleaned[line_no][stack_id];

				if c.is_ascii_whitespace() {
					//dont want to include a whitespace trailer + there wont be any more crates in this stack anyway
					break 'next;
				}
				stack.push(c);
			}

			stacks.push(stack);
		}

		Ok(Shipyard { stacks })
	}
}

impl FromStr for Instruction {
	type Err = Infallible; //TODO

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut splitspace = s.split_ascii_whitespace().fuse();
		if let (Some("move"), Some(count_str), Some("from"), Some(src_str), Some("to"), Some(dst_str)) =
			(splitspace.next(), splitspace.next(), splitspace.next(), splitspace.next(), splitspace.next(), splitspace.next())
		{
			return Ok(Instruction {
				count: count_str.parse::<usize>().unwrap(), //TODO
				src: src_str.parse::<usize>().unwrap() - 1, //TODO
				dst: dst_str.parse::<usize>().unwrap() - 1, //TODO
			});
		}

		panic!("unexpected item in bagging area") //TODO
	}
}

fn run_a_on(input: String) -> impl Display {
	let mut lines = input.lines();
	let mut shipyard = Shipyard::from_lines_iterator(&mut lines).expect("unexpected item in bagging area"); //TODO
	for line in lines {
		if line.is_empty() {
			continue;
		}

		if let Ok(insn) = Instruction::from_str(line) {
			shipyard.run_instruction(&insn);
		}
	}

	shipyard.answer()
}

fn run_b_on(input: String) -> impl Display {
	let mut lines = input.lines();
	let mut shipyard = Shipyard::from_lines_iterator(&mut lines).expect("unexpected item in bagging area"); //TODO
	for line in lines {
		if line.is_empty() {
			continue;
		}

		if let Ok(insn) = Instruction::from_str(line) {
			shipyard.run_instruction_9001(&insn);
		}
	}

	shipyard.answer()
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(5))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(5))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(5)).to_string(), "CMZ");
		assert_eq!(run_b_on(test_input_as_string(5)).to_string(), "MCD");
	}

	#[test]
	fn real() {
		//I was hoping it would spell something
		assert_eq!(run_a().to_string(), "FWSHSPJWM");
		assert_eq!(run_b().to_string(), "PWPWHGFZS");
	}
}
