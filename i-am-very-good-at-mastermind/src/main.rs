#![feature(array_windows)]

use std::ops::{Index, IndexMut};

use anyhow::Result;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() -> Result<()> {
	let table = FrequencyTable::build(&load_word_list()?);

	table.print_frequency_table();
	table.print_sorted_alphabets();
	table.print_repeated_letter_table();
	table.print_digraph_table();

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
	fn count(&mut self, c: char) {
		self[c] += 1;
	}
	
	fn count_word(&mut self, word: &str) {
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
	
	pub fn print_repeated_letter_table(&self) {
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
	
	pub fn print_digraph_table(&self) {
		let mut all_digraphs: LetterTable<LetterTable<usize>> = Default::default();
		let mut start_digraphs: LetterTable<LetterTable<usize>> = Default::default();
		let mut mid_digraphs: LetterTable<LetterTable<usize>> = Default::default();
		let mut end_digraphs: LetterTable<LetterTable<usize>> = Default::default();
		
		for word in &self.words {
			let word = word.chars().collect::<Vec<char>>(); // Me and the boys writing zero cost abstractions.
			for &[a, b] in word.array_windows::<2>() {
				all_digraphs[a][b] += 1;
			}
			start_digraphs[word[0]][word[1]] += 1;
			mid_digraphs[word[1]][word[2]] += 1;
			end_digraphs[word[2]][word[3]] += 1;
		}
		
		//find the widest number, so i can use something shorter than \t to separate the table columns
		//yeah i should impl iter for lettertable huh
		
		fn print_table(table: &LetterTable<LetterTable<usize>>) {
			let mut widest = 0;
			for x in ALPHABET.chars() {
				for &y in &table[x].0 {
					if y > widest {
						widest = y;
					}
				}
			}
			let cell_width = widest.to_string().len() + 1;
			
			//print top row
			let top_space: String = " ".repeat(cell_width - 1);
			println!("  {}", ALPHABET.chars().map(String::from).collect::<Vec<_>>().join(&top_space)); //gahhhhh
			
			//print table body
			for first in ALPHABET.chars() {
				print!("{} ", first);
				for next in ALPHABET.chars() {
					let n = table[first][next];
					let n_str = n.to_string();
					print!("{}", n_str);
					print!("{}", " ".repeat(cell_width - n_str.len()));
				}
				println!();
			}
		}
		
		println!("! left: first letter, top: second letter !");
		println!("     == ALL DIGRAPHS ==");
		print_table(&all_digraphs);
		println!("     == FIRST LETTER -> SECOND LETTER ==");
		print_table(&start_digraphs);
		println!("     == SECOND LETTER -> THIRD LETTER ==");
		print_table(&mid_digraphs);
		println!("     == THIRD LETTER -> FOURTH LETTER ==");
		print_table(&end_digraphs);
		println!();
	}
}
