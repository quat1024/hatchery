#![allow(unused_variables)] //for now

use serde::ser;
use serde::ser::Impossible;
use serde::serde_if_integer128;
use std::fmt;
use thiserror::Error;

pub fn to_string<T>(value: &T) -> Result<String, SerErr> where T: serde::Serialize {
	let mut ser = Ser {
		indent: 0,
		out: "".into()
	};
	value.serialize(&mut ser)?;
	Ok(ser.out)
}

pub struct Ser {
	indent: usize,
	out: String,
}

impl Ser {
	fn quoted_string(&mut self, s: &str) -> Result<(), SerErr> {
		self.out.push('"');
		//TODO: Escape it!
		self.out.push_str(s);
		self.out.push('"');
		Ok(())
	}
	
	fn empty_string(&mut self) {
		self.out.push_str("\"\"");
	}
	
	fn render_indent(&mut self) {
		for _ in 0..self.indent {
			self.out.push('\t');
		}
	}
}

macro_rules! serialize_as_quoted_string {
	( $func:ident $type:ty ) => {
		fn $func(self, v: $type) -> Result<Self::Ok, Self::Error> {
			self.quoted_string(&v.to_string())
		}
	}
}

impl<'a> ser::Serializer for &'a mut Ser {
	type Ok = ();
	type Error = SerErr;

	//Just make it be quiet for now, lol
	type SerializeSeq = Impossible<(), Self::Error>;
	type SerializeTuple = Impossible<(), Self::Error>;
	type SerializeTupleStruct = Impossible<(), Self::Error>;
	type SerializeTupleVariant = Impossible<(), Self::Error>;
	type SerializeMap = Impossible<(), Self::Error>;
	type SerializeStruct = Self;
	type SerializeStructVariant = Impossible<(), Self::Error>;
	
	serialize_as_quoted_string!(serialize_bool bool);
	serialize_as_quoted_string!(serialize_i8 i8);
	serialize_as_quoted_string!(serialize_i16 i16);
	serialize_as_quoted_string!(serialize_i32 i32);
	serialize_as_quoted_string!(serialize_i64 i64);
	serialize_as_quoted_string!(serialize_u8 u8);
	serialize_as_quoted_string!(serialize_u16 u16);
	serialize_as_quoted_string!(serialize_u32 u32);
	serialize_as_quoted_string!(serialize_u64 u64);
	serialize_as_quoted_string!(serialize_f32 f32);
	serialize_as_quoted_string!(serialize_f64 f64);
	
	serde_if_integer128! {
		serialize_as_quoted_string!(serialize_i128 i128);
		serialize_as_quoted_string!(serialize_u128 u128);
	}
	
	serialize_as_quoted_string!(serialize_char char);
	
	//This is zero-copyable probably if i wanna be cool
	serialize_as_quoted_string!(serialize_str &str);

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!("bytes!")
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		//Discard the wrapper, as is conventional with json-like formats.
		self.serialize_unit()
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		//Discard the wrapper, as is conventional with json-like formats.
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		//Use the empty string.
		self.empty_string();
		Ok(())
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		//Discard the wrapper and ignore its name, as is convention with unit structs.
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		//Just record the name.
		self.serialize_str(variant)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		//Discard the wrapper, as is convention with newtype structs.
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
		T: serde::Serialize,
	{
		todo!("newtype variant")
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		todo!("seq")
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
		todo!("map")
	}

	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		self.out += "{\n";
		self.indent += 1;
		
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

impl<'a> ser::SerializeStruct for &'a mut Ser {
    type Ok = ();
    type Error = SerErr;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize {
        self.render_indent();
		self.quoted_string(key)?;
		self.out += "\t";
		value.serialize(&mut **self)?;
		self.out += "\n";
		Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.indent -= 1;
		self.render_indent();
		self.out += "}\n";
		Ok(())
    }
}

#[derive(Error, Debug)]
pub enum SerErr {
	#[error("{0}")]
	Message(String),
}

impl ser::Error for SerErr {
	fn custom<T>(msg: T) -> Self
	where
		T: fmt::Display,
	{
		SerErr::Message(msg.to_string())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn serialize_simple_structs() {
		#[derive(serde::Serialize)]
		struct Hello {
			hello: String,
			my_name_is: String,
			small_number: u8,
			medium_number: i32,
			big_number: u64,
			yea: Yea
		}
		
		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: String,
			wooo_yeah: bool
		}
		
		let hello = Hello {
			hello: "world".into(),
			my_name_is: "jeff".into(),
			small_number: 69,
			medium_number: 621,
			big_number: 123456789,
			yea: Yea {
				yeah_woo: "yeah woo yea woooo yeah woo".into(),
				wooo_yeah: true
			}
		};
		
		let result = to_string(&hello).unwrap();
		println!("{}", result);
	}
}