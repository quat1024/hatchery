//! THIS IS THE PART WHERE I REGRET USING RUST

use std::collections::HashMap;

use crate::*;

#[derive(Clone, Debug)]
enum Instruction {
	NavigateUp,
	NavigateToRoot,
	NavigateDown { name: String },
	Listing(Vec<LsEntry>),
}

#[derive(Clone, Debug)]
enum LsEntry {
	Dir { name: String },
	File { name: String, size: usize },
}

//trees babey
//TODO: i tried to make this not take owned Strings but code hard
#[derive(Debug, Default, Clone)]
struct Directory {
	subdirs: HashMap<String, Directory>,
	files: HashMap<String, usize>,
}

impl Directory {
	//TODO dont take ownership of insns (code hard)
	fn build(insns: Vec<Instruction>) -> Directory {
		let mut root: Directory = Directory { subdirs: HashMap::new(), files: HashMap::new() };

		let mut current_path: Vec<String> = Vec::new();

		for insn in insns {
			match insn {
				Instruction::NavigateUp => {
					current_path.pop();
				},
				Instruction::NavigateToRoot => {
					current_path.clear();
				},
				Instruction::NavigateDown { name } => {
					current_path.push(name.clone()); //TODO dont clone
				},
				Instruction::Listing(ls) => {
					for lsentry in ls {
						match lsentry {
							LsEntry::File { name, size } => {
								root.get_or_create_path(&current_path).files.insert(name, size);
							},
							LsEntry::Dir { name } => {
								//ehh ??
							},
						}
					}
				},
			}
		}

		root
	}

	fn get_or_create_subdir(&mut self, subdir_name: &str) -> &mut Directory {
		//TODO: remove clone!!!
		self.subdirs.entry(subdir_name.to_owned()).or_insert(Default::default())
	}

	fn get_or_create_path(&mut self, path: &Vec<String>) -> &mut Directory {
		let mut cursor = self;
		for path_element in path {
			cursor = cursor.get_or_create_subdir(path_element);
		}
		cursor
	}

	// todo there is probably a general graph-traversal method i should write

	fn total_size(&self) -> usize {
		self.files.values().sum::<usize>() + self.subdirs.values().map(Self::total_size).sum::<usize>()
	}

	fn flatten(&self) -> Vec<&Directory> {
		let mut flat = Vec::new();
		self.flatten_impl(&mut flat);
		flat
	}

	fn flatten_impl<'a>(&'a self, flat: &mut Vec<&'a Directory>) {
		flat.push(self);
		for subdir in self.subdirs.values() {
			subdir.flatten_impl(flat)
		}
	}
}

impl Instruction {
	fn parse_insn_list(input: String) -> Vec<Instruction> {
		let mut instructions = Vec::new();

		let mut lineserator = input.lines().fuse().peekable();
		while let Some(line) = lineserator.next() {
			if line == "$ cd /" {
				instructions.push(Instruction::NavigateToRoot);
			} else if line == "$ cd .." {
				instructions.push(Instruction::NavigateUp);
			} else if let Some(path) = line.strip_prefix("$ cd ") {
				instructions.push(Instruction::NavigateDown { name: path.to_owned() })
			} else if line == "$ ls" {
				//now the fun part!
				let mut ls_entry_lines = Vec::new();
				loop {
					//if the next line starts with $, break out immediately so the next loop iteration picks it up
					if let Some(upcoming) = lineserator.peek() {
						if upcoming.starts_with('$') {
							break;
						}
					}

					//else we need to consume the line
					if let Some(line2) = lineserator.next() {
						if let Some(dirname) = line2.strip_prefix("dir ") {
							ls_entry_lines.push(LsEntry::Dir { name: dirname.to_owned() })
						} else if let Some((size_unparsed, filename)) = line2.split_once(' ') {
							ls_entry_lines.push(LsEntry::File { name: filename.to_owned(), size: size_unparsed.parse().expect("alskjdklasjdkljaskd") })
							//TODO
						}
					} else {
						break;
					}
				}

				instructions.push(Instruction::Listing(ls_entry_lines))
			} else {
				panic!("unexpected item in bagging area"); //TODO
			}
		}

		instructions
	}
}

pub fn a(input: String) -> impl Display {
	let directory = Directory::build(Instruction::parse_insn_list(input));

	directory.flatten().iter().filter(|dir| dir.total_size() <= 100_000).map(|dir| dir.total_size()).sum::<usize>().to_string()
}

pub fn b(input: String) -> impl Display {
	let directory = Directory::build(Instruction::parse_insn_list(input));

	let disk_size = 70_000_000;
	let update_size = 30_000_000;

	let used_size = directory.total_size();
	let unused_size = disk_size - used_size;

	let mut winning_dir_total_size = usize::MAX;
	for dir in directory.flatten() {
		let dir_total_size = dir.total_size();
		let unused_size_after_deletion = unused_size + dir_total_size;
		if unused_size_after_deletion > update_size && winning_dir_total_size > dir_total_size {
			winning_dir_total_size = dir_total_size;
		}
	}

	winning_dir_total_size.to_string()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(test_input_as_string(7)).to_string(), "95437");
		assert_eq!(b(test_input_as_string(7)).to_string(), "24933642");
	}

	#[test]
	fn real() {
		assert_eq!(a(input_as_string(7)).to_string(), "1454188");
		assert_eq!(b(input_as_string(7)).to_string(), "4183246");
	}
}
