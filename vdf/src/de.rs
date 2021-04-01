use serde::de;
use serde::serde_if_integer128;
use serde::Deserialize;

use crate::error::VdfErr;

struct VdfDeserializer<'de> {
	input: &'de str,
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, VdfErr>
where
	T: Deserialize<'a>,
{
	let mut de = VdfDeserializer::from_str(input);
	T::deserialize(&mut de)
}

impl<'de> VdfDeserializer<'de> {
	pub fn from_str(input: &'de str) -> Self {
		VdfDeserializer { input }
	}

	fn skip_whitespace_and_comments(&mut self) {
		'again: loop {
			//Snip off leading whitespace, including newlines.
			self.input = self.input.trim_start();

			//Check if a comment starts here
			if self.input.starts_with("//") {
				//Double-slash comment. Skip until after the end of the line
				if let Some((_, cdr)) = self.input.split_once('\n') {
					self.input = cdr;
				} else {
					//Or maybe the comment is ended by EOF, that's ok too
					self.input = "";
				}

				continue 'again; //proceed to skip more whitespace in the line after the comment
			}

			break;
		}
	}

	fn yoink(&mut self) -> Result<&'de str, VdfErr> {
		self.skip_whitespace_and_comments();

		let mut chars = self.input.chars();

		match chars.next().ok_or(VdfErr::EndOfFile)? {
			'"' => {
				//Starts with a double quote, so we are reading a double quoted string.
				//TODO this method fucking sucks LMAO

				//Find the end of the double-quoted string.
				//first munch this " character
				self.input = &self.input[1..];
				//... so that find() finds the ending character instead
				let ending_quote_pos = self.input.find('"').ok_or(VdfErr::EndOfFile)?; //todo nicer error messages more clear about the reason would be nice
				self.input = &self.input[ending_quote_pos + 1..];
				Ok(&self.input[0..ending_quote_pos])
			}
			_ => {
				//Does not start with a double quote.
				//Find the end of the unquoted string (first non-whitespace character).
				//TODO: Allow values (not keys) to run into the end of the line
				if let Some((car, cdr)) = self.input.split_once(|c: char| c.is_whitespace()) {
					self.input = cdr;
					Ok(car)
				} else {
					//The value runs all the way up to the end of the line.
					//Common in toy examples like parsing "123" as an entire vdf file, I guess
					//Just return all the input
					Ok(self.input)
				}
			}
		}
	}
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut VdfDeserializer<'de> {
	type Error = VdfErr;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_any")
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s {
			"1" | "true" => visitor.visit_bool(true),
			"0" | "false" => visitor.visit_bool(false),
			_ => Err(VdfErr::ParseBool(s.to_string())),
		}
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<i8>() {
			Ok(i) => visitor.visit_i8(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<i16>() {
			Ok(i) => visitor.visit_i16(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<i32>() {
			Ok(i) => visitor.visit_i32(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<i64>() {
			Ok(i) => visitor.visit_i64(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<u8>() {
			Ok(i) => visitor.visit_u8(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<u16>() {
			Ok(i) => visitor.visit_u16(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<u32>() {
			Ok(i) => visitor.visit_u32(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<u64>() {
			Ok(i) => visitor.visit_u64(i),
			Err(e) => Err(VdfErr::ParseInt(e)),
		}
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<f32>() {
			Ok(f) => visitor.visit_f32(f),
			Err(e) => Err(VdfErr::ParseFloat(e)),
		}
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<f64>() {
			Ok(f) => visitor.visit_f64(f),
			Err(e) => Err(VdfErr::ParseFloat(e)),
		}
	}

	serde_if_integer128! {
		fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
		where
			V: de::Visitor<'de>
		{
			let s = self.yoink()?;
			match s.parse::<i128>() {
				Ok(i) => visitor.visit_i128(i),
				Err(e) => Err(VdfErr::ParseInt(e))
			}
		}

		fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
		where
			V: de::Visitor<'de>
		{
			let s = self.yoink()?;
			match s.parse::<u128>() {
				Ok(i) => visitor.visit_u128(i),
				Err(e) => Err(VdfErr::ParseInt(e))
			}
		}
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		match s.parse::<char>() {
			Ok(c) => visitor.visit_char(c),
			Err(e) => Err(VdfErr::ParseChar(e)),
		}
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		visitor.visit_str(s)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.yoink()?;
		visitor.visit_string(s.to_string())
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_bytes")
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_byte_buf")
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_option")
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_unit")
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_unit_struct")
	}

	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_newtype_struct")
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_seq")
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_tuple")
	}

	fn deserialize_tuple_struct<V>(
		self,
		name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_tuple_struct")
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_map")
	}

	fn deserialize_struct<V>(
		self,
		name: &'static str,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_struct")
	}

	fn deserialize_enum<V>(
		self,
		name: &'static str,
		variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_enum")
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_identifier")
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("deserialize_ignored_any")
	}
}

mod test {
	use super::*;

	#[test]
	fn de_u64() {
		//TODO: make the tests pass! D:

		assert_eq!(from_str::<u64>("123").expect("failed to parse 1"), 123);
		//assert_eq!(from_str::<u64>("\"123\"").expect("failed to parse 2"), 123); //panics
		//assert_eq!(from_str::<u64>("\"123\"asdf").expect("failed to parse 3"), 123); //int parse failure, but it shouldn't
		assert_eq!(from_str::<u64>("   123  ").expect("failed to parse 4"), 123);
	}
}
