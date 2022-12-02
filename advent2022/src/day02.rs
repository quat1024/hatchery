use std::error::Error;

use crate::input_as_string;

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

pub fn run_a() -> Result<(), Box<dyn Error>> {
	let score = input_as_string("02.txt")
		.lines()
		.map(|line| {
			let linesplit = line.split_ascii_whitespace().collect::<Vec<_>>();
			let theirs = Rps::from_str(linesplit[0]);
			let mine = Rps::from_str(linesplit[1]);

			mine.score_against(&theirs)
		})
		.sum::<u64>();

	println!("score is {}", score);
	Ok(())
}

//15442
pub fn run_b() -> Result<(), Box<dyn Error>> {
	let score = input_as_string("02.txt")
		.lines()
		.map(|line| {
			let linesplit = line.split_ascii_whitespace().collect::<Vec<_>>();
			let theirs = Rps::from_str(linesplit[0]);
			let mine = theirs.produce_win_state(&WinState::from_str(linesplit[1]));

			mine.score_against(&theirs)
		})
		.sum::<u64>();

	println!("score is {}", score);

	Ok(())
}

