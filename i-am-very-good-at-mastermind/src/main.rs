use anyhow::Result;
use std::collections::HashMap;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() -> Result<()> {
	let table = FrequencyTable::build(&load_word_list()?);
	
	table.print_frequency_table();
	table.print_sorted_alphabets();
	
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

struct FrequencyTable {
	pub positional: [HashMap<char, usize>; 4],
	pub anywhere: HashMap<char, usize>
}

impl FrequencyTable {
	pub fn build(words: &[String]) -> FrequencyTable {
		let mut positional = [ HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];
		let mut anywhere = HashMap::new();
		
		for word in words {
			for (idx, c) in word.char_indices() {
				*positional[idx].entry(c).or_default() += 1;
				*anywhere.entry(c).or_default() += 1;
			}
		}
		
		FrequencyTable { positional, anywhere }
	}
	
	pub fn count_for_position(&self, c: char, pos: usize) -> usize {
		// Weirdchamp
		self.positional[pos].get(&c).copied().unwrap_or_default()
	}
	
	pub fn total_count(&self, c: char) -> usize {
		self.anywhere.get(&c).copied().unwrap_or_default()
	}
	
	pub fn print_frequency_table(&self) {
		println!("letter\t1\t2\t3\t4\tanywhere");
		for c in ALPHABET.chars() {
			println!("{}\t{}\t{}\t{}\t{}\t{}", c,
				self.count_for_position(c, 0),
				self.count_for_position(c, 1),
				self.count_for_position(c, 2),
				self.count_for_position(c, 3),
				self.total_count(c)
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
		
		println!("letter 1 frequency order:\t{}", sort_alphabet(&|c| self.count_for_position(c, 0)));
		println!("letter 2 frequency order:\t{}", sort_alphabet(&|c| self.count_for_position(c, 1)));
		println!("letter 3 frequency order:\t{}", sort_alphabet(&|c| self.count_for_position(c, 2)));
		println!("letter 4 frequency order:\t{}", sort_alphabet(&|c| self.count_for_position(c, 3)));
		println!("all letters frequency order:\t{}", sort_alphabet(&|c| self.total_count(c)));
	}
}