#![feature(array_windows)]

use std::ops::{Index, IndexMut};

use anyhow::Result;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() -> Result<()> {
	let table = FrequencyTable::build(&load_word_list()?);

	table.print_frequency_table();
	table.print_sorted_alphabets();
	table.print_repeated_letter_stats();

	Ok(())
}

fn load_word_list() -> Result<Vec<String>> {
	Ok(std::fs::read_to_string("./i-am-very-good-at-mastermind/libwords-mastermind.txt")?
		.lines()
		.filter_map(|line| {
			// pick out the bit between the double-quotes since each word is quoted in the word list.
			if let (Some(start), Some(end)) = (line.find('"'), line.rfind('"')) {
				Some(line[start + 1..end].to_owned())
			} else {
				None
			}
		})
		.collect())
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LetterTable<T>([T; 26]);

impl<T> Index<char> for LetterTable<T> {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        &self.0[(index as usize) - ('a' as usize)]
    }
}

impl<T> IndexMut<char> for LetterTable<T> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        &mut self.0[(index as usize) - ('a' as usize)]
    }
}

impl LetterTable<usize> {
	fn new() -> LetterTable<usize> {
		Default::default()
	}
	
	fn count(&mut self, c: char) {
		self[c] += 1;
	}
	
	fn count_word(&mut self, word: &String) {
		for c in word.chars() {
			self.count(c);
		}
	}
}

struct FrequencyTable {
	pub positional: [LetterTable<usize>; 4],
	pub anywhere: LetterTable<usize>,
	pub words: Vec<String>
}

impl FrequencyTable {
	
	
	pub fn build(words: &[String]) -> FrequencyTable {
		let mut positional: [LetterTable<usize>; 4] = Default::default();
		let mut anywhere: LetterTable<usize> = Default::default();

		let words: Vec<String> = words.into();
		
		for word in &words {
			anywhere.count_word(word);
			for (idx, c) in word.char_indices() {
				positional[idx].count(c);
			}
		}

		FrequencyTable { positional, anywhere, words }
	}

	pub fn print_frequency_table(&self) {
		println!("letter\t1\t2\t3\t4\tanywhere");
		for c in ALPHABET.chars() {
			println!(
				"{}\t{}\t{}\t{}\t{}\t{}",
				c,
				self.positional[0][c],
				self.positional[1][c],
				self.positional[2][c],
				self.positional[3][c],
				self.anywhere[c]
			)
		}
		println!();
	}

	pub fn print_sorted_alphabets(&self) {
		fn sort_alphabet(sort_key: &dyn Fn(char) -> usize) -> String {
			let mut alphabet_vec: Vec<char> = ALPHABET.chars().collect();
			alphabet_vec.sort_by_key(|c| sort_key(*c));
			alphabet_vec.reverse(); // weirdchamp
						//alphabet_vec.concat()
						//alphabet_vec.join("")
			alphabet_vec.into_iter().collect::<String>()
		}

		println!("letter 1 frequency order:\t{}", sort_alphabet(&|c| self.positional[0][c]));
		println!("letter 2 frequency order:\t{}", sort_alphabet(&|c| self.positional[1][c]));
		println!("letter 3 frequency order:\t{}", sort_alphabet(&|c| self.positional[2][c]));
		println!("letter 4 frequency order:\t{}", sort_alphabet(&|c| self.positional[3][c]));
		println!("all letters frequency order:\t{}", sort_alphabet(&|c| self.anywhere[c]));
		println!()
	}
	
	pub fn print_repeated_letter_stats(&self) {
		let mut doubles_frequency: LetterTable<usize> = Default::default();
		let mut adjacent_doubles_frequency: LetterTable<usize> = Default::default();
		let mut triples_frequency: LetterTable<usize> = Default::default();
		let mut triples_words: LetterTable<Vec<&str>> = Default::default(); // because theres so few i might as well print them
		
		for word in &self.words {
			let mut word_frequency: LetterTable<usize> = Default::default();
			word_frequency.count_word(word);
			
			for c in ALPHABET.chars() {
				if word_frequency[c] == 2 {
					doubles_frequency.count(c);
				} else if word_frequency[c] == 3 {
					triples_frequency.count(c);
					triples_words[c].push(word);
				}
			}
			
			let word = word.chars().collect::<Vec<char>>(); // Me and the boys writing zero cost abstractions.
			for &[a, b] in word.array_windows::<2>() {
				if a == b {
					adjacent_doubles_frequency.count(a);
				}
			}
		}
		
		println!("letter\tdoubles\tadjdubs\ttriples");
		for c in ALPHABET.chars() {
			println!(
				"{}\t{}\t{}\t{}\t{}",
				c,
				doubles_frequency[c],
				adjacent_doubles_frequency[c],
				triples_frequency[c],
				triples_words[c].join(", ")
			)
		}
		println!();
	}
}
