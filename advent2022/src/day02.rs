use std::fmt::Display;

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
		match self {
			Rps::Rock => match other {
				Rps::Rock => WinState::Tie,
				Rps::Paper => WinState::Loss,
				Rps::Scissors => WinState::Win,
			},
			Rps::Paper => match other {
				Rps::Rock => WinState::Win,
				Rps::Paper => WinState::Tie,
				Rps::Scissors => WinState::Loss,
			},
			Rps::Scissors => match other {
				Rps::Rock => WinState::Loss,
				Rps::Paper => WinState::Win,
				Rps::Scissors => WinState::Tie,
			},
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
		match self {
			Rps::Rock => match state {
				WinState::Win => Rps::Paper,
				WinState::Loss => Rps::Scissors,
				WinState::Tie => Rps::Rock,
			},
			Rps::Paper => match state {
				WinState::Win => Rps::Scissors,
				WinState::Loss => Rps::Rock,
				WinState::Tie => Rps::Paper,
			},
			Rps::Scissors => match state {
				WinState::Win => Rps::Rock,
				WinState::Loss => Rps::Paper,
				WinState::Tie => Rps::Scissors,
			},
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

fn run_a_on(input: String) -> impl Display {
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

fn run_b_on(input: String) -> impl Display {
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

pub fn run_a() -> impl Display {
	run_a_on(input_as_string(2))
}

pub fn run_b() -> impl Display {
	run_b_on(input_as_string(2))
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_a_on(test_input_as_string(2)).to_string(), "15");
		assert_eq!(run_b_on(test_input_as_string(2)).to_string(), "12");
	}

	#[test]
	fn real() {
		assert_eq!(run_a().to_string(), "15422");
		assert_eq!(run_b().to_string(), "15442");
	}
}
