#![allow(unused_variables)] //for now

use serde::{Serializer, ser};
use serde::ser::Impossible;
use serde::serde_if_integer128;
use std::fmt;
use thiserror::Error;

pub fn to_string<T>(value: &T) -> Result<String, SerializeErr>
where
	T: serde::Serialize,
{
	let mut ser = VdfSerializer::default();
	value.serialize(&mut ser)?;
	Ok(ser.out)
}

pub struct VdfSerializer {
	out: String,
	state: State,
	indent_depth: usize,
	key_quote_rule: KeyQuoteRule
}

impl Default for VdfSerializer {
    fn default() -> Self {
        VdfSerializer {
			out: String::new(),
			state: State::WaitingForKey,
			indent_depth: 0,
			key_quote_rule: KeyQuoteRule::Always
		}
    }
}

enum KeyQuoteRule {
	Always,
	NotBlocks,
	Never
}

enum State {
	WaitingForKey,
	WaitingForValue(String), //TODO: If I can do this with &str, that would be great.
	WritingNestedValue(usize)
}

impl Default for State {
    fn default() -> Self {
        State::WaitingForKey
    }
}

impl VdfSerializer {
	fn indent(&mut self) {
		for i in 0..self.indent_depth {
			self.out.push('\t');
		}
	}
	
	#[inline]
	fn increase_indent(&mut self) {
		self.indent_depth += 1;
	}
	
	#[inline]
	fn decrease_indent(&mut self) {
		self.indent_depth -= 1;
	}
	
	#[inline]
	fn newline(&mut self) {
		self.out.push('\n');
	}
	
	fn write_key(&mut self, key: &str, is_block: bool) {
		let quote_key = match self.key_quote_rule {
		    KeyQuoteRule::Always => true,
		    KeyQuoteRule::NotBlocks => !is_block,
		    KeyQuoteRule::Never => false
		};
		
		if quote_key {
			self.out.push('"');
		}
		
		self.out.push_str(key);
		
		if quote_key {
			self.out.push('"');
		}
	}
	
	fn write_value(&mut self, value: &str) {
		//check quotation options, escape it maybe, etc
		self.out.push('"');
		self.out.push_str(value);
		self.out.push('"');
	}
	
	fn begin_block(&mut self) {
		//Move out of the enum variant while resetting it to WaitingForKey. Nasty nasty nasty.
		if let State::WaitingForValue(_) = &self.state {
			match std::mem::take(&mut self.state) {
				State::WaitingForValue(key) => {
					self.indent();
					self.write_key(&key, true);
				},
				_ => unreachable!()
			}
		}
		
		self.newline();
		self.indent();
		self.out.push('{');
		self.increase_indent();
		self.newline();
	}
	
	// Call after writing the last value inside this block.
	fn end_block(&mut self) {
		self.decrease_indent();
		self.indent();
		self.out.push('}');
		self.newline();
	}
	
	fn accept_str(&mut self, s: &str) -> Result<(), SerializeErr> {
		match &self.state {
			State::WaitingForKey => {
				self.state = State::WaitingForValue(s.to_string())
			}
		    State::WaitingForValue(_) => {
				match std::mem::take(&mut self.state) {
					State::WaitingForValue(key) => {
						self.indent();
						self.write_key(&key, false);
						self.out.push('\t');
						self.write_value(s);
						self.newline();
					},
					_ => unreachable!()
				}
			}
		    State::WritingNestedValue(depth) => {
				todo!("nested values NYI")
			}
		}
		
		Ok(())
	}
}

macro_rules! use_to_string {
	( $func:ident $type:ty ) => {
		fn $func(self, v: $type) -> Result<Self::Ok, Self::Error> {
			self.accept_str(&v.to_string())
		}
	};
}

impl<'a> ser::Serializer for &'a mut VdfSerializer {
	type Ok = ();
	type Error = SerializeErr;

	type SerializeSeq = VdfSeqSerializer<'a>;
	type SerializeTuple = Impossible<(), Self::Error>;
	type SerializeTupleStruct = Impossible<(), Self::Error>;
	type SerializeTupleVariant = Impossible<(), Self::Error>;
	type SerializeMap = Self;
	type SerializeStruct = Self;
	type SerializeStructVariant = Impossible<(), Self::Error>;
	
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.accept_str(if v { "true" } else { "false" })
	}
	
	use_to_string!(serialize_i8 i8);
	use_to_string!(serialize_i16 i16);
	use_to_string!(serialize_i32 i32);
	use_to_string!(serialize_i64 i64);
	use_to_string!(serialize_u8 u8);
	use_to_string!(serialize_u16 u16);
	use_to_string!(serialize_u32 u32);
	use_to_string!(serialize_u64 u64);
	use_to_string!(serialize_f32 f32);
	use_to_string!(serialize_f64 f64);

	serde_if_integer128! {
		use_to_string!(serialize_i128 i128);
		use_to_string!(serialize_u128 u128);
	}

	use_to_string!(serialize_char char);
	
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.accept_str(v)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!("bytes")
    }

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.accept_str("")
    }

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        value.serialize(self)
    }

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.accept_str("()")
    }

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!("unit struct")
    }

	fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!("unit variant")
    }

	fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        self.serialize_str(name)?;
		value.serialize(self)
    }

	fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize {
        todo!("newtype variant")
    }

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.begin_block();
		Ok(VdfSeqSerializer {
			ser: self,
			index: 0
		})
    }

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!("tuple")
    }

	fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!("tuple struct")
    }

	fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!("tuple variant")
    }

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.begin_block();
		Ok(self)
    }

	fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.begin_block();
		Ok(self)
    }

	fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!("struct variant")
    }
}

impl<'a> ser::SerializeStruct for &'a mut VdfSerializer {
    type Ok = ();
    type Error = SerializeErr;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
		self.accept_str(key)?;
		value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_block();
		Ok(())
    }
}

pub struct VdfSeqSerializer<'a> {
	ser: &'a mut VdfSerializer,
	index: u32
}

impl<'a> ser::SerializeSeq for VdfSeqSerializer<'a> {
	type Ok = ();
    type Error = SerializeErr;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        self.ser.serialize_u32(self.index)?;
		value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end_block();
		Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut VdfSerializer {
    type Ok = ();
    type Error = SerializeErr;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
		value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_block();
		Ok(())
    }
}

#[derive(Error, Debug)]
pub enum SerializeErr {
	#[error("{0}")]
	Message(String),
}

impl ser::Error for SerializeErr {
	fn custom<T>(msg: T) -> Self
	where
		T: fmt::Display,
	{
		SerializeErr::Message(msg.to_string())
	}
}

mod test {
	use super::*;
	use serde::Serialize;
	use std::collections::HashMap;

	#[test]
	fn serialize_simple_structs() {
		#[derive(serde::Serialize)]
		struct Hello {
			hello: String,
			my_name_is: String,
			small_number: u8,
			medium_number: i32,
			big_number: u64,
			yea: Yea,
		}

		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: String,
			wooo_yeah: bool,
		}

		let hello = Hello {
			hello: "world".into(),
			my_name_is: "jeff".into(),
			small_number: 69,
			medium_number: 621,
			big_number: 123456789,
			yea: Yea {
				yeah_woo: "yeah woo yea woooo yeah woo".into(),
				wooo_yeah: true,
			},
		};

		let result = to_string(&hello).unwrap();
		println!("{}", result);
	}
	
	#[test]
	fn im_the_map() {
		let mut themap: HashMap<&'static str, char> = HashMap::new();
		themap.insert("ayy", 'a');
		themap.insert("bee", 'b');
		themap.insert("see", 'c');
		
		let themap = to_string(&themap).unwrap();
		
		println!("{}", themap);
	}
	
	#[test]
	fn seq() {
		let funny = vec!["asd", "ghj", "ahdjahds"];
		println!("{}", to_string(&funny).unwrap());
	}
	
	#[test]
	fn seq_of_structs() {
		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: &'static str,
			wooo_yeah: u32,
		}
		
		let funny = vec![
			Yea {
				yeah_woo: "yeah woo!",
				wooo_yeah: 123
			},
			Yea {
				yeah_woo: "yasdadeah woo!",
				wooo_yeah: 12345
			},
			Yea {
				yeah_woo: "yeah woasdadasdo!",
				wooo_yeah: 12345678
			},
		];
		
		println!("{}", to_string(&funny).unwrap());
	}
	
	#[test]
	fn newtype_struct() {
		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: &'static str,
			wooo_yeah: u32,
		}
		
		#[derive(serde::Serialize)]
		struct Wrapper(Yea);
		
		let funny = Wrapper(Yea{
			yeah_woo: "yeah woo",
			wooo_yeah: 69420
		});
		
		println!("{}", to_string(&funny).unwrap());
	}
}