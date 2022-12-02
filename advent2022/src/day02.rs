use std::error::Error;

use crate::input_as_string;

enum Rps {
	Rock, Paper, Scissors
}

enum WinState {
	Win, Loss, Tie
}

impl Rps {
	fn from_str(s: &str) -> Rps {
		match s.chars().next() {
			Some('A') | Some('X') => Rps::Rock,
			Some('B') | Some('Y') => Rps::Paper,
			Some('C') | Some('Z') => Rps::Scissors,
			_ => panic!()
		}
	}
	
	fn pair_from_str(s: &str) -> (Rps, Rps) {
		let stringsplit = s.split_ascii_whitespace().collect::<Vec<_>>();
		(Self::from_str(stringsplit[0]), Self::from_str(stringsplit[1]))
	}
	
	fn wins_against(&self, other: &Rps) -> WinState {
		match self {
			Rps::Rock => match other {
				Rps::Rock => WinState::Tie,
				Rps::Paper => WinState::Loss,
				Rps::Scissors => WinState::Win
			},
			Rps::Paper => match other {
				Rps::Rock => WinState::Win,
				Rps::Paper => WinState::Tie,
				Rps::Scissors => WinState::Loss
			},
			Rps::Scissors => match other {
				Rps::Rock => WinState::Loss,
				Rps::Paper => WinState::Win,
				Rps::Scissors => WinState::Tie
			}
		}
	}
	
	fn score_against(&self, other: &Rps) -> u64 {
		self.wins_against(other).score() + match self {
			Rps::Rock => 1,
			Rps::Paper => 2,
			Rps::Scissors => 3,
		}
	}
}

impl WinState {
	fn score(&self) -> u64 {
		match self {
			WinState::Win => 6,
			WinState::Loss => 0,
			WinState::Tie => 3
		}
	}
}

pub fn run_a() -> Result<(), Box<dyn Error>> {
	let input = input_as_string("02.txt");
	
	//parse strategy guide
	let strategy_guide = input.lines().map(Rps::pair_from_str).collect::<Vec<_>>();
	
	//everything goes according to plan
	let mut score = 0;
	for entry in strategy_guide {
		score += &entry.1.score_against(&entry.0);
	}
	
	println!("score is {}", score);
	
	//not 11548
	//not 15026
	Ok(())
}

pub fn run_b() -> Result<(), Box<dyn Error>> {
	let input = input_as_string("01.txt");
	
	Ok(())
}