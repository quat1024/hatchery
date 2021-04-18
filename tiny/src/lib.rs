use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TinyMappings<'a> {
	classes: HashMap<&'a str, &'a str>,
	fields: HashMap<Member<'a>, &'a str>,
	methods: HashMap<Member<'a>, &'a str>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Member<'a> {
	owner: &'a str,
	descriptor: &'a str,
	name: &'a str,
}

impl<'a> TinyMappings<'a> {
	pub fn read(input: &'a str, src: &'a str, dst: &'a str) -> Result<TinyMappings<'a>, String> {
		//todo use better error type
		let mut lines = input.lines();

		let header = lines.next().ok_or("no header")?;
		let mut header_parts = header.split('\t');

		//Verify tinyv1 header
		match header_parts.next() {
			Some("v1") => {}
			Some(other) => return Err(format!("invalid header {}", other)),
			None => return Err("missing header parts".into()),
		}

		//Pick out the mappings of interest from the header
		let mut src_index: usize = 999999;
		let mut dst_index: usize = 999999;

		let header_parts = header_parts.enumerate();
		for (idx, s) in header_parts {
			if s == src {
				src_index = idx;
			}
			if s == dst {
				dst_index = idx;
			}
			if src_index != 999999 && dst_index != 999999 {
				break;
			}
		}

		if src_index == 999999 {
			return Err(format!("could not find src mapping set {} in file", src));
		}

		if dst_index == 999999 {
			return Err(format!("could not find dst mapping set {} in file", dst));
		}

		//rebind as immutable
		let src_index = src_index;
		let dst_index = dst_index;

		//Start parsing the file
		let mut map = TinyMappings {
			classes: HashMap::new(),
			fields: HashMap::new(),
			methods: HashMap::new(),
		};
		
		let mut zeroth_to_src: HashMap<&'a str, &'a str> = HashMap::new(); //doesn't allocate yet

		for line in lines {
			let mut split = line.split('\t');
			if let Some(first) = split.next() {
				//This can be done without allocation, but it's very hard :(
				let rest = split.collect::<Vec<_>>();
					
				match first {
					"CLASS" => {
						map.classes.insert(rest[src_index], rest[dst_index]);
						
						if src_index != 0 {
							zeroth_to_src.insert(rest[0], rest[src_index]);
						}
					}
					"FIELD" | "METHOD" => {
						//extra comparison?
						let target = if first == "METHOD" { &mut map.methods } else { &mut map.fields };
						
						let owner = rest[0];
						let descriptor = rest[1];
						let rest = &rest[2..];
						
						let src_name = rest[src_index];
						let dst_name = rest[dst_index];
						
						target.insert(
							Member {
								owner,
								descriptor,
								name: src_name
							},
							dst_name
						);
					}
					_ => (), //skip
				}
			}
		}
		
		if src_index != 0 {
			//todo this is a garbo fire
			
			//class names inside field owners, field descriptors, method owners, and method descriptors must be remapped
			map.fields = map.fields.iter().map(|(field, name)| {
				let remapped_member = Member {
					owner: map.classes.get(field.owner).unwrap_or(&field.owner),
					descriptor: field.descriptor, //todo
					name: field.name,
				};
				
				(remapped_member, *name)
			}).collect();
			
			map.methods = map.methods.iter().map(|(method, name)| {
				let remapped_member = Member {
					owner: map.classes.get(method.owner).unwrap_or(&method.owner),
					descriptor: method.descriptor, //todo
					name: method.name,
				};
				
				(remapped_member, *name)
			}).collect();
		}

		Ok(map)
	}
	
	pub fn map_class(&self, class: &'_ str) -> Option<&&'a str> {
		self.classes.get(class)
	}
}

#[allow(unused_imports)]
mod test {
	use super::*;
	
	#[allow(dead_code)] //r-a being overzealous; used in the tests
	fn for_each_permutation<F>(a: &'static str, b: &'static str, c: &'static str, f: F) where F: Fn(&'static str, &'static str) {
		f(a, b);
		f(b, a);
		f(a, c);
		f(c, a);
		f(b, c);
		f(c, b);
	}
	
	#[test]
	fn read_small_tiny() {
		let file_contents = "\
v1	spongy	fluffy	tasty
CLASS	a	aFluffy	class_tasty_a
CLASS	b	bFluffy	class_tasty_b
FIELD	a	Z	a	aFluffyField	field_tasty_a
FIELD	a	Z	b	bFluffyField	field_tasty_b
FIELD	b	Z	c	cFluffyField	field_tasty_c
FIELD	b	Z	d	dFluffyField	field_tasty_d
METHOD	a	()Z	a	aFluffyMethod	method_tasty_a
METHOD	a	(Z)Z	b	bFluffyMethod	method_tasty_b
METHOD	b	(J)V	c	cFluffyMethod	method_tasty_c
METHOD	b	()V	d	dFluffyMethod	method_tasty_d";

		for_each_permutation("spongy", "fluffy", "tasty", |a, b| {
			let a_to_b = TinyMappings::read(&file_contents, a, b).expect("a to b");
			let b_to_a = TinyMappings::read(&file_contents, b, a).expect("b to a");
			
			//classes are inverses of each other
			let inverse_a_to_b_classes = a_to_b.classes.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<&'_ str, &'_ str>>();
			assert_eq!(inverse_a_to_b_classes, b_to_a.classes);
			
			let inverse_b_to_a_classes = b_to_a.classes.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<&'_ str, &'_ str>>();
			assert_eq!(inverse_b_to_a_classes, a_to_b.classes);
			
			//todo check more inverses
		});
	}
	
	#[test]
	fn ordering() {
		let out_of_order = "\
v1	spongy	fluffy	tasty
FIELD	a	Z	a	aFluffyField	field_tasty_a
METHOD	a	(La;)Z	a	aFluffyMethod	method_tasty_a
CLASS	a	aFluffy	class_tasty_a_asdad";

		let in_order = "\
v1	spongy	fluffy	tasty
CLASS	a	aFluffy	class_tasty_a_asdad
FIELD	a	Z	a	aFluffyField	field_tasty_a
METHOD	a	(La;)Z	a	aFluffyMethod	method_tasty_a";
		
		for_each_permutation("spongy", "fluffy", "tasty", |a, b| {
			let out_of_order = TinyMappings::read(&out_of_order, a, b).expect("out of order");
			let in_order = TinyMappings::read(&in_order, a, b).expect("in order");
		
			assert_eq!(out_of_order, in_order);
		});
	}
	
	#[test]
	fn parse_production_tiny() {
		println!("(times are cumulative)");
		
		let start_time = std::time::Instant::now();
		
		let file_contents = std::fs::read_to_string("res/yarn-tiny-21w15a.tiny").expect("couldnt read file");
		println!("reading entire file to string: {:?}", std::time::Instant::now() - start_time);
		
		for_each_permutation("official", "intermediary", "named", |a, b| {
			let a_to_b = TinyMappings::read(&file_contents, a, b).expect("a to b");
			println!("parsing as {} to {}: {:?}", a, b, std::time::Instant::now() - start_time);
			let b_to_a = TinyMappings::read(&file_contents, b, a).expect("b to a");
			println!("parsing as {} to {}: {:?}", b, a, std::time::Instant::now() - start_time);
			
			//classes are inverses of each other
			let inverse_a_to_b_classes = a_to_b.classes.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<&'_ str, &'_ str>>();
			assert_eq!(inverse_a_to_b_classes, b_to_a.classes);
			
			let inverse_b_to_a_classes = b_to_a.classes.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<&'_ str, &'_ str>>();
			assert_eq!(inverse_b_to_a_classes, a_to_b.classes);
			
			//todo check more inverses
		});
		
		println!("elapsed time: {:?}", std::time::Instant::now() - start_time);
	}
}
