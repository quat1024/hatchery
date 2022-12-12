use crate::*;

#[derive(Clone, Copy)]
enum Rps {
	Rock,
	Paper,
	Scissors,
}

enum WinState {
	Win,
	Loss,
	Tie,
}

impl Rps {
	fn from_str(s: &str) -> Rps {
		match s.chars().next() {
			Some('A') | Some('X') => Rps::Rock,
			Some('B') | Some('Y') => Rps::Paper,
			Some('C') | Some('Z') => Rps::Scissors,
			_ => panic!("unexpected item in bagging area"),
		}
	}

	fn wins_against(&self, other: &Rps) -> WinState {
		match (self, other) {
			(Rps::Rock, Rps::Rock) => WinState::Tie,
			(Rps::Rock, Rps::Paper) => WinState::Loss,
			(Rps::Rock, Rps::Scissors) => WinState::Win,
			(Rps::Paper, Rps::Rock) => WinState::Win,
			(Rps::Paper, Rps::Paper) => WinState::Tie,
			(Rps::Paper, Rps::Scissors) => WinState::Loss,
			(Rps::Scissors, Rps::Rock) => WinState::Loss,
			(Rps::Scissors, Rps::Paper) => WinState::Win,
			(Rps::Scissors, Rps::Scissors) => WinState::Tie,
		}
	}

	fn score_against(&self, other: &Rps) -> u64 {
		(match self {
			Rps::Rock => 1,
			Rps::Paper => 2,
			Rps::Scissors => 3,
		}) + self.wins_against(other).score()
	}

	fn produce_win_state(&self, state: &WinState) -> Rps {
		match (self, state) {
			(Rps::Rock, WinState::Win) => Rps::Paper,
			(Rps::Rock, WinState::Loss) => Rps::Scissors,
			(Rps::Rock, WinState::Tie) => Rps::Rock,
			(Rps::Paper, WinState::Win) => Rps::Scissors,
			(Rps::Paper, WinState::Loss) => Rps::Rock,
			(Rps::Paper, WinState::Tie) => Rps::Paper,
			(Rps::Scissors, WinState::Win) => Rps::Rock,
			(Rps::Scissors, WinState::Loss) => Rps::Paper,
			(Rps::Scissors, WinState::Tie) => Rps::Scissors,
		}
	}
}

impl WinState {
	fn score(&self) -> u64 {
		match self {
			WinState::Win => 6,
			WinState::Loss => 0,
			WinState::Tie => 3,
		}
	}

	fn from_str(s: &str) -> WinState {
		match s.chars().next() {
			Some('X') => WinState::Loss,
			Some('Y') => WinState::Tie,
			Some('Z') => WinState::Win,
			_ => panic!("unexpected item in bagging area"),
		}
	}
}

pub fn a(input: &str) -> impl Display {
	input
		.lines()
		.map(|line| {
			let linesplit = line.split_ascii_whitespace().collect::<Vec<_>>();
			let theirs = Rps::from_str(linesplit[0]);
			let mine = Rps::from_str(linesplit[1]);

			mine.score_against(&theirs)
		})
		.sum::<u64>()
}

pub fn b(input: &str) -> impl Display {
	input
		.lines()
		.map(|line| {
			let linesplit = line.split_ascii_whitespace().collect::<Vec<_>>();
			let theirs = Rps::from_str(linesplit[0]);
			let mine = theirs.produce_win_state(&WinState::from_str(linesplit[1]));

			mine.score_against(&theirs)
		})
		.sum::<u64>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(2)).to_string(), "15");
		assert_eq!(b(&test_input_as_string(2)).to_string(), "12");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(2)).to_string(), "15422");
		assert_eq!(b(&input_as_string(2)).to_string(), "15442");
	}
}
