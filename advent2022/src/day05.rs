use std::convert::Infallible;
use std::str::FromStr;

use crate::*;

struct Shipyard {
	///zero-indexed (unlike the puzzle input)
	///stacks grow upwards, the bottom of each pile is element 0
	stacks: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
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
	
	fn run_instruction(&mut self, insn: Instruction) {
		let src = insn.src - 1; //instructions are one-indexed
		let dst = insn.dst - 1;
		
		for _ in 0..insn.count {
			if let Some(crate_label) = self.stacks[src].pop() {
				self.stacks[dst].push(crate_label);
			} else {
				panic!("pop from empty stack? state rn is {}", self.answer())
			}
		}
	}
	
	fn run_instruction_9001(&mut self, insn: Instruction) {
		let src = &mut self.stacks[insn.src - 1]; //instructions are one-indexed
		
		//you cant borrow both at once for some stupid reason so i will have to collect into a structure first
		let mut shit = Vec::<char>::new();
		for what in &src[src.len() - insn.count..] {
			shit.push(*what);
		}
		
		//then remove the end with this clumsy method, there's gotta be a nicer way to do this one btw
		for _ in 0..insn.count {
			src.pop();
		}
		
		self.stacks[insn.dst - 1].append(&mut shit);
	}
}

impl FromStr for Shipyard {
	type Err = Infallible; //TODO

	fn from_str(shipyard_str: &str) -> Result<Self, Self::Err> {
		//(step 1) remove all the whitespace and the 1 2 3 footer and stuff
		let mut lines_cleaned = Vec::<Vec<char>>::new();

		'done: for line in shipyard_str.lines() {
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
				count: count_str.parse().unwrap(), //TODO
				src: src_str.parse().unwrap(),   //TODO
				dst: dst_str.parse().unwrap(),   //TODO
			});
		}

		panic!("unexpected item in bagging area") //TODO
	}
}

fn split_into_shipyard_and_instructions<'a>(input: &'a str) -> (&'a str, &'a str) {
	let double_newline = input.find("\n\n").or_else(|| input.find("\r\n\r\n"));
	let double_newline = double_newline.expect("couldnt find separator :("); //TODO actual error handling

	return input.split_at(double_newline);
}

fn run_a_on(input: String) -> impl Display {
	let (shipyard_unparsed, instructions_unparsed) = split_into_shipyard_and_instructions(&input);

	//parse shipyard
	let mut shipyard = Shipyard::from_str(shipyard_unparsed).expect("unexpected item in bagging area");
	
	//parse instructions
	let instructions = instructions_unparsed.lines().filter_map(|line| {
		let trim = line.trim();
		if trim.is_empty() {
			None
		} else {
			Some(trim)
		}
	}).map(Instruction::from_str).collect::<Result<Vec<Instruction>, _>>().unwrap();

	//perform each instruction on the shipyard
	for insn in instructions {
		shipyard.run_instruction(insn);
	}
	
	shipyard.answer()
}

fn run_b_on(input: String) -> impl Display {
	let (shipyard_unparsed, instructions_unparsed) = split_into_shipyard_and_instructions(&input);

	//parse shipyard
	let mut shipyard = Shipyard::from_str(shipyard_unparsed).expect("unexpected item in bagging area");
	
	//parse instructions
	let instructions = instructions_unparsed.lines().filter_map(|line| {
		let trim = line.trim();
		if trim.is_empty() {
			None
		} else {
			Some(trim)
		}
	}).map(Instruction::from_str).collect::<Result<Vec<Instruction>, _>>().unwrap();

	//perform each instruction on the shipyard
	for insn in instructions {
		shipyard.run_instruction_9001(insn);
	}
	
	shipyard.answer()
}

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(5))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(5))
}

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
