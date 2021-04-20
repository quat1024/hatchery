#[derive(Debug)]
struct VmfObject<'a> {
	items: Vec<Kv<'a>>,
}

#[derive(Debug)]
struct Kv<'a> {
	key: &'a str,
	value: Value<'a>,
}

#[derive(Debug)]
enum Value<'a> {
	Str(&'a str),
	Obj(VmfObject<'a>),
}

impl<'a> VmfObject<'a> {
	fn new() -> Self {
		VmfObject { items: Vec::new() }
	}

	fn print(&self) {
		self.items.iter().for_each(|i| {
			i.print();
			println!()
		});
	}
}

impl<'a> From<Vec<Kv<'a>>> for VmfObject<'a> {
	fn from(vec: Vec<Kv<'a>>) -> Self {
		VmfObject { items: vec }
	}
}

impl<'a> From<Kv<'a>> for VmfObject<'a> {
	fn from(k: Kv<'a>) -> Self {
		VmfObject { items: vec![k] }
	}
}

impl<'a> Kv<'a> {
	fn new_str(key: &'a str, value: &'a str) -> Self {
		Kv { key, value: Value::Str(value) }
	}

	fn new_obj<I>(key: &'a str, value: I) -> Self
	where
		I: Into<VmfObject<'a>>,
	{
		Kv { key, value: Value::Obj(value.into()) }
	}

	fn print(&self) {
		print!("\"");
		print!("{}", self.key);
		print!("\"");
		match &self.value {
			Value::Str(s) => {
				print!("\t\"");
				print!("{}", s);
				print!("\"");
			},
			Value::Obj(o) => {
				print!("\n{{");
				o.print();
				println!("}}");
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn asdf() {
		let my_key_1 = Kv::new_str("k1", "my value");
		let my_key_2 = Kv::new_str("k2", "my other value");

		let obj: VmfObject = Kv::new_obj("My Keys", vec![my_key_1, my_key_2]).into();

		obj.print()
	}
}
