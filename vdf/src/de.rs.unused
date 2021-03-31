use serde::de;
use thiserror::Error;

pub struct Deserializer<'de> {
	input: &'de str,
}

impl<'de> Deserializer<'de> {
	fn peek_char(&'de self) -> Result<char, DeErr> {
		self.input.chars().next().ok_or(DeErr::End)
	}

	fn peel_count(&'de mut self, count: Option<usize>) -> Result<&'de str, DeErr> {
		match count {
			Some(count) => {
				let s = &self.input[..count];
				self.input = &self.input[count + 1..];
				Ok(s)
			}
			None => Err(DeErr::End),
		}
	}

	fn peel_possibly_quoted_str(&'de mut self) -> Result<&'de str, DeErr> {
		if self.peek_char()? == '"' {
			//quoted string
			self.peel_quoted_str()
		} else {
			//unquoted string
			self.peel_count(self.input.find(|c: char| c.is_whitespace()))
		}
	}
	
	fn peel_quoted_str(&'de mut self) -> Result<&'de str, DeErr> {
		if self.peek_char()? == '"' {
			self.input = &self.input[1..];
			//TODO parse escapes
			self.peel_count(self.input.find('"'))
		} else {
			Err(DeErr::ExpectQuote)
		}
	}
	
	fn munch_empty_string(&'de mut self) -> Result<(), DeErr> {
		let mut chars = self.input.chars();
		
		let first = chars.next().ok_or(DeErr::End)?;
		let next = chars.next().ok_or(DeErr::End)?;
		
		if first == '"' && next == '"' {
			Ok(())
		} else {
			Err(DeErr::ExpectEmptyString)
		}
	}
	
	fn munch_whitespace(&'de mut self) -> Result<(), DeErr> {
		let _ = self.peel_count(self.input.find(|c: char| !c.is_whitespace()))?;
		Ok(())
	}
}

#[derive(Error, Debug)]
pub enum DeErr {
	#[error("{0}")]
	Message(String),
	#[error("Unexpected end of file")]
	End,
	#[error("Cannot use deserialize_any")]
	NoAny,
	#[error("Could not parse {0}")]
	Parse(&'static str),
	#[error("Expected a single character, but found {0}")]
	ExpectSingleCharacter(String),
	#[error("Expected quoted string")]
	ExpectQuote,
	#[error("Expected an empty string \"\"")]
	ExpectEmptyString
}

impl serde::de::Error for DeErr {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		DeErr::Message(msg.to_string())
	}
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> where 'a: 'de {
	type Error = DeErr;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		Err(DeErr::NoAny)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_bool(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("bool"))?)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i8(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("i8"))?)
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i16(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("i16"))?)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i32(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("i32"))?)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_i64(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("i64"))?)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u8(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("u8"))?)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u16(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("u16"))?)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u32(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("u32"))?)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_u64(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("u64"))?)
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_f32(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("f32"))?)
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_f64(self.peel_possibly_quoted_str()?.parse().map_err(|_| DeErr::Parse("f64"))?)
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let s = self.peel_possibly_quoted_str()?;
		match s.chars().next() {
			Some(c) => visitor.visit_char(c),
			None => Err(DeErr::ExpectSingleCharacter(s.to_string()))
		}
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_borrowed_str(self.peel_possibly_quoted_str()?)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_borrowed_str(self.peel_possibly_quoted_str()?)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("bytes")
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("bytebuf")
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("option")
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let _ = self.munch_empty_string()?;
		visitor.visit_unit()
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("unit struct")
	}

	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("newtype struct")
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("seq")
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("tuple")
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
		todo!("tuple struct")
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("map")
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
		todo!("struct")
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
		todo!("enum")
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		todo!("ident")
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		Err(DeErr::NoAny)
	}
}

struct KeyValue<'a, 'de: 'a> {
	d: &'a mut Deserializer<'de>,
	key: bool
}

impl<'a, 'de> de::MapAccess<'de> for KeyValue<'a, 'de> where 'a : 'de {
    type Error = DeErr;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de> {
        self.d.munch_whitespace()?;
		seed.deserialize(self.d).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de> {
        todo!()
    }
}