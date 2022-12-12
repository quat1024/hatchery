use crate::*;

enum Insn {
	Noop,
	Addx(isize),
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

pub fn a(input: &str) -> impl Display {
	let mut x = 1;
	let mut cycle_count = 0;
	let mut answer = 0;

	let mut tick = |x: &isize| {
		cycle_count += 1;
		if [20, 60, 100, 140, 180, 220].contains(&cycle_count) {
			answer += *x * cycle_count;
		}
	};

	for insn in input.lines().map(Insn::parse) {
		match insn {
			Insn::Noop => {
				tick(&x);
			},
			Insn::Addx(arg) => {
				tick(&x);
				tick(&x);
				x += arg;
			},
		}
	}

	answer
}

pub fn b(input: &str) -> impl Display {
	let mut x: isize = 1;
	let mut cycle_count: isize = 0;

	let mut screen = String::new();

	let mut tick = |x: &isize| {
		let raster_pos = cycle_count % 40;
		cycle_count += 1;

		screen.push(if raster_pos.abs_diff(*x) <= 1 { '#' } else { '.' });
		if raster_pos == 39 {
			screen.push('\n');
		}
	};

	for insn in input.lines().map(Insn::parse) {
		match insn {
			Insn::Noop => {
				tick(&x);
			},
			Insn::Addx(arg) => {
				tick(&x);
				tick(&x);
				x += arg;
			},
		}
	}

	screen
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(10)).to_string(), "13140");

		assert_eq!(
			b(&test_input_as_string(10)).to_string(),
			"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
		);
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(10)).to_string(), "12520");

		assert_eq!(
			b(&input_as_string(10)).to_string(),
			"####.#..#.###..####.###....##..##..#....
#....#..#.#..#....#.#..#....#.#..#.#....
###..####.#..#...#..#..#....#.#....#....
#....#..#.###...#...###.....#.#.##.#....
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.#....####.#.....##...###.####.
"
		);
	}
}
