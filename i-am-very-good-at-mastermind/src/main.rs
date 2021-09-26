use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
	let mut frequency_tables = vec![
		HashMap::<char, usize>::new(),
		HashMap::new(),
		HashMap::new(),
		HashMap::new()
	];
	let mut total_table = HashMap::<char, usize>::new();
	
	for word in load_word_list()? {
		for (idx, c) in word.char_indices() {
			*frequency_tables[idx].entry(c).or_default() += 1;
			*total_table.entry(c).or_default() += 1;
		}
	}
	
	println!("letter\t1\t2\t3\t4\ttotal");
	for c in "abcdefghijklmnopqrstuvwxyz".chars() {
		// Weirdchamp
		println!("{}\t{}\t{}\t{}\t{}\t{}", c,
			frequency_tables[0].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[1].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[2].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[3].get(&c).map(|x| *x).unwrap_or_default(),
			total_table.get(&c).map(|x| *x).unwrap_or_default()
		)
	}
	
	Ok(())
}

fn load_word_list() -> Result<Vec<String>> {
	Ok(std::fs::read_to_string("./i-am-very-good-at-mastermind/libwords-mastermind.txt")?
		.lines()
		.filter_map(|line| {
			if line.is_empty() {
				None
			} else {
				let start = line.find('"').expect("i want quote");
				let end = line.rfind('"').expect("gimme the quote");
				
				Some(line[start + 1..end].to_owned())
			}
		})
		.collect())
}