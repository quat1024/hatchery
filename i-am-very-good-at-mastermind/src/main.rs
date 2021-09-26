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
	
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
	let alphabet_vec: Vec<char> = alphabet.chars().collect();
	
	// print frequency table
	println!("letter\t1\t2\t3\t4\tanywhere");
	for c in alphabet.chars() {
		// Weirdchamp
		println!("{}\t{}\t{}\t{}\t{}\t{}", c,
			frequency_tables[0].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[1].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[2].get(&c).map(|x| *x).unwrap_or_default(),
			frequency_tables[3].get(&c).map(|x| *x).unwrap_or_default(),
			total_table.get(&c).map(|x| *x).unwrap_or_default()
		)
	}
	println!("\n");
	
	// sort alphabet by letter frequency for each position
	for i in 0..=4 {
		// this is really bad lmfao
		let target = if i == 4 { &total_table } else { &frequency_tables[i] };
		let name = if i == 4 { "all letters frequency order:".to_owned() } else { format!("letter {} frequency order:", i) };
		
		let mut alphabet_vec = alphabet_vec.clone();
		alphabet_vec.sort_by_key(|c| target.get(&c).map(|x| *x).unwrap_or_default());
		alphabet_vec.reverse(); // weirdchamp
		println!("{}\t{}", name, alphabet_vec.iter().collect::<String>());
	}
	
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