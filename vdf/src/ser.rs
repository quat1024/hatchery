//! VDF serialization.

#![allow(unused_variables)] //for now

use std::fmt;

use serde::ser;
use serde::ser::Impossible;
use serde::serde_if_integer128;
use serde::Serializer;
use thiserror::Error;

//a lot of these doc comments are cribbed from serde-json lol. Hey, ecosystem-wide consistency is good.

/// Serialize the given data structure as a VDF String.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to fail.
pub fn to_string<T>(value: &T) -> Result<String, SerializeErr>
where
	T: serde::Serialize,
{
	let mut ser = VdfSerializer::default();
	value.serialize(&mut ser)?;
	Ok(ser.out)
}

/// Serialize the given data structure as a VDF String, where the name of the top-level block is provided.
///
/// For example, if T serializes as:
/// ```text
/// {
///     "key" "value"
/// }
/// ```
/// then calling `to_string_with_toplevel_block(t, "Block")` will produce the following:
/// ```text
/// "Block"
/// {
///     "key" "value"
/// }
/// ```
pub fn to_string_with_toplevel_block<T>(
	value: &T,
	toplevel_block_name: &str,
) -> Result<String, SerializeErr>
where
	T: serde::Serialize,
{
	let mut ser = VdfSerializer::default();
	ser.serialize_str(toplevel_block_name)?;
	value.serialize(&mut ser)?;
	Ok(ser.out)
}

/// Thing what do the seralize!!!
pub struct VdfSerializer {
	pub out: String, //todo should this be public lol. Really I should use a Writer
	state: State,
	indent_depth: usize,
	format_settings: FormatSettings,
}

impl VdfSerializer {
	/// Create a serializer with the given Settings.
	/// 
	/// Note that `VdfSerializer::default` can be used to create one with the default `editoritems.txt`-like settings.
	pub fn with_settings(settings: FormatSettings) -> Self {
		VdfSerializer {
			out: String::new(),
			state: State::WaitingForKey,
			indent_depth: 0,
			format_settings: settings,
		}
	}
}

impl Default for VdfSerializer {
	fn default() -> Self {
		VdfSerializer {
			out: String::new(),
			state: State::WaitingForKey,
			indent_depth: 0,
			format_settings: FormatSettings::p2c_like(),
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
/// Settings that control the look of the generated VDF file.
pub struct FormatSettings {
	/// How should keys be quoted?
	key_quote_rule: KeyQuoteRule,
	/// If `Some`, `key_quote_rule` is overridden with this for top-level keys.
	toplevel_key_quote_rule: Option<KeyQuoteRule>,
	/// How should values be quoted?
	value_quote_rule: ValueQuoteRule,
	/// How should booleans be serialized?
	bool_format: BoolFormat,
	/// If `true`, curly braces are placed at the same indentation level as their contents, emulating how BEEMOD writes vdf files.
	bump_braces: bool,
	/// The string used for indentation. Must be some kind of whitespace. One copy of the string is output for each indentation level.
	indent_str: &'static str,
	/// The string used to space out keys and values. Must be some kind of whitespace.
	inter_str: &'static str,
}

impl FormatSettings {
	/// A new `FormatSettings` that formats files like a Hammer .vmf map file.
	/// 
	/// * Keys quoted only if they don't precede blocks
	/// * Values always quoted.
	/// * Numeric bools.
	/// * Tab indentation.
	/// * Spaces between keys and values.
	pub fn vmf_like() -> Self {
		FormatSettings {
			key_quote_rule: KeyQuoteRule::NotBlocks,
			toplevel_key_quote_rule: None,
			value_quote_rule: ValueQuoteRule::Always,
			bool_format: BoolFormat::Numeric,
			bump_braces: false,
			indent_str: "\t",
			inter_str: " "
		}
	}

	/// A new `FormatSettings` that formats files like a Portal 2 .p2c puzzle file.
	/// 
	/// * Keys and values always quoted.
	/// * Numeric bools.
	/// * Tab indentation.
	/// * Tabs separate keys and values.
	/// 
	/// This is also how the vanilla `editoritems.txt` is formatted, and seems to be the format that a lot of Valve's more modern stuff uses,
	/// like steam controller .vdf files.
	pub fn p2c_like() -> Self {
		FormatSettings {
			key_quote_rule: KeyQuoteRule::Always,
			toplevel_key_quote_rule: None,
			value_quote_rule: ValueQuoteRule::Always,
			bool_format: BoolFormat::Numeric,
			bump_braces: false,
			indent_str: "\t",
			inter_str: "\t",
		}
	}
	
	/// A new `FormatSettings` that formats files in a way resembling BEEMOD's vdf serializer.
	/// 
	/// * Keys and values always quoted.
	/// * Numeric bools.
	/// * Tab indentation.
	/// * Spaces separate keys and values.
	/// * Opening and closing curly braces gain an extra level of indentation.
	pub fn beemod_like() -> Self {
		FormatSettings {
			key_quote_rule: KeyQuoteRule::Always,
			toplevel_key_quote_rule: None,
			value_quote_rule: ValueQuoteRule::Always,
			bool_format: BoolFormat::Numeric,
			bump_braces: true,
			indent_str: "\t",
			inter_str: " ",
		}
	}

	/// A new `FormatSettings` that formats files like a Source Engine gameinfo.txt data file. It's not entirely accurate...
	/// 
	/// * Keys never quoted.
	/// * Nonnumeric values quoted.
	/// * Tab indentation.
	/// * Tabs separate keys and values.
	pub fn gameinfo_like() -> Self {
		FormatSettings {
			key_quote_rule: KeyQuoteRule::Never,
			toplevel_key_quote_rule: Some(KeyQuoteRule::Always),
			value_quote_rule: ValueQuoteRule::Nonnumeric, //TODO: gameinfo.txt also doesn't quote SearchPaths
			bool_format: BoolFormat::Numeric,
			bump_braces: false,
			indent_str: "\t",
			inter_str: "\t"
		}
	}
}

/// How should keys be quoted? Note that keys with whitespace are always quoted.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum KeyQuoteRule {
	/// Always insert double quotes around keys.
	Always,
	/// Insert double quotes if the corresponding value is an immediate value, not a block. Like Hammer VMFs.
	NotBlocks,
	/// Never insert double quotes if they can be avoided.
	Never,
}

/// How should values (in the key-value store) be quoted? Note that values with whitespace are always quoted.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum ValueQuoteRule {
	/// Always insert double quotes around values.
	Always,
	/// Insert double quotes around values that aren't numbers.
	Nonnumeric,
	/// Never insert double quotes if they can be avoided.
	Never,
}

/// How should `bool` values be serialized?
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub enum BoolFormat {
	/// Bools will be serialized as the string "true" or "false".
	Word,
	/// Bools will be serialized as the number "1" or "0".
	Numeric,
}

enum State {
	WaitingForKey,
	WaitingForValue(String), //TODO: If I can do this with &str, that would be great.
	WritingNestedValue(usize),
}

impl Default for State {
	fn default() -> Self {
		State::WaitingForKey
	}
}

impl VdfSerializer {
	fn indent(&mut self) {
		for i in 0..self.indent_depth {
			self.out.push_str(self.format_settings.indent_str);
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
		//Spaghetti lmao I'm sorry.
		
		let top = &self.format_settings.toplevel_key_quote_rule;
		let rule = if top.is_some() && self.indent_depth == 0 {
			//Unwrap safety: is_some was just checked above
			&top.as_ref().unwrap()
		} else {
			&self.format_settings.key_quote_rule
		};
		
		let quote_key = *rule == KeyQuoteRule::Always
			|| (*rule == KeyQuoteRule::NotBlocks && !is_block)
			|| key.is_empty()
			|| key.chars().any(|c| c.is_ascii_whitespace());

		if quote_key {
			self.out.push('"');
			self.out.push_str(key);
			self.out.push('"');
		} else {
			self.out.push_str(key);
		}
	}

	fn write_value(&mut self, value: &str, numeric: bool) {
		let rule = &self.format_settings.value_quote_rule;
		
		let quote_value = *rule == ValueQuoteRule::Always
			|| (*rule == ValueQuoteRule::Nonnumeric && !numeric)
			|| value.is_empty()
			|| value.chars().any(|c| c.is_ascii_whitespace());
		
		if quote_value {
			self.out.push('"');
			self.out.push_str(value);
			self.out.push('"');
		} else {
			self.out.push_str(value);
		}
		
	}

	fn begin_block(&mut self) {
		self.state = match std::mem::take(&mut self.state) {
			State::WaitingForValue(key) => {
				self.indent();
				self.write_key(&key, true);

				State::WaitingForKey
			}
			other => other, //unchanged
		};

		self.newline();
		if self.format_settings.bump_braces {
			self.increase_indent();
			self.indent();
			self.out.push('{');
		} else {
			self.indent();
			self.out.push('{');
			self.increase_indent();
		}
		
		self.newline();
	}

	//call after writing the last value inside this block.
	fn end_block(&mut self) {
		if self.format_settings.bump_braces {
			self.indent();
			self.decrease_indent();
		} else {
			self.decrease_indent();
			self.indent();
		}
		
		self.out.push('}');
		self.newline();
	}

	fn accept_str(&mut self, s: &str, numeric: bool) -> Result<(), SerializeErr> {
		self.state = match std::mem::take(&mut self.state) {
			State::WaitingForKey => State::WaitingForValue(s.to_string()),
			State::WaitingForValue(key) => {
				self.indent();
				self.write_key(&key, false);
				self.out.push_str(self.format_settings.inter_str);
				self.write_value(s, numeric);
				self.newline();

				State::WaitingForKey
			}
			State::WritingNestedValue(depth) => {
				todo!("nested values NYI")
			}
		};

		Ok(())
	}
}

macro_rules! use_to_string {
	( $func:ident | $type:ty | $numeric:literal) => {
		fn $func(self, v: $type) -> Result<Self::Ok, Self::Error> {
			self.accept_str(&v.to_string(), $numeric)
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
		match self.format_settings.bool_format {
			BoolFormat::Word => self.serialize_str(if v { "true" } else { "false" }),
			BoolFormat::Numeric => self.serialize_u8(if v { 1 } else { 0 }),
		}
	}

	use_to_string!(serialize_i8 | i8 | true);
	use_to_string!(serialize_u8 | u8 | true);
	use_to_string!(serialize_i16 | i16 | true);
	use_to_string!(serialize_u16 | u16 | true);
	use_to_string!(serialize_i32 | i32 | true);
	use_to_string!(serialize_u32 | u32 | true);
	use_to_string!(serialize_f32 | f32 | true);
	use_to_string!(serialize_i64 | i64 | true);
	use_to_string!(serialize_u64 | u64 | true);
	use_to_string!(serialize_f64 | f64 | true);

	serde_if_integer128! {
		use_to_string!(serialize_i128 | i128 | true);
		use_to_string!(serialize_u128 | u128 | true);
	}

	use_to_string!(serialize_char | char | false);

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.accept_str(v, false)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!("bytes")
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_str("")
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_str("")
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
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
		self.serialize_str(name)?; //TODO maybe make this an option (separate from the formatter options)
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
		self.begin_block();
		Ok(VdfSeqSerializer {
			ser: self,
			index: 0,
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
		T: serde::Serialize,
	{
		self.serialize_str(key)?;
		value.serialize(&mut **self)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.end_block();
		Ok(())
	}
}

pub struct VdfSeqSerializer<'a> {
	ser: &'a mut VdfSerializer,
	index: u32,
}

impl<'a> ser::SerializeSeq for VdfSeqSerializer<'a> {
	type Ok = ();
	type Error = SerializeErr;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: serde::Serialize,
	{
		self.ser.serialize_u32(self.index)?;
		self.index += 1;
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
		T: serde::Serialize,
	{
		key.serialize(&mut **self)
	}

	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: serde::Serialize,
	{
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

#[allow(unused_imports)] //RLS bug? These imports are used in the tests below
mod test {
	use std::collections::HashMap;

	use serde::Serialize;

	use super::*;
	use crate::named_seq_func;

	
	//N.B All of these are "toy" tests that just println some things and always pass, instead of actually testing anything, right now.
	//Run with `cargo test -- --show-output`
	
	#[test]
	fn toy_serialize_simple_structs() {
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
	fn toy_im_the_map() {
		//N.B. Random-between-runs (!) hash order means actually testing this is tricky, lol
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
	fn toy_seq_of_structs() {
		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: &'static str,
			wooo_yeah: u32,
		}

		let funny = vec![
			Yea {
				yeah_woo: "yeah woo!",
				wooo_yeah: 123,
			},
			Yea {
				yeah_woo: "yasdadeah woo!",
				wooo_yeah: 12345,
			},
			Yea {
				yeah_woo: "yeah woasdadasdo!",
				wooo_yeah: 12345678,
			},
		];

		println!("{}", to_string(&funny).unwrap());
	}

	#[test]
	fn toy_newtype_struct() {
		#[derive(serde::Serialize)]
		struct Yea {
			yeah_woo: &'static str,
			wooo_yeah: u32,
		}

		#[derive(serde::Serialize)]
		struct Wrapper(Yea);

		let funny = Wrapper(Yea {
			yeah_woo: "yeah woo",
			wooo_yeah: 69420,
		});

		println!("{}", to_string(&funny).unwrap());
	}

	#[test]
	fn toy_named_seq() {
		named_seq_func!(steve "steve");

		#[derive(serde::Serialize)]
		struct Hey {
			name: &'static str,
			#[serde(serialize_with = "steve")]
			#[serde(flatten)]
			things: Vec<Thing>,
		}

		#[derive(serde::Serialize)]
		struct Thing {
			abc: u32,
			xyz: u32,
		}

		let hey = Hey {
			name: "Name!!!!!!!",
			things: vec![
				Thing { abc: 30, xyz: 30 },
				Thing {
					abc: 70,
					xyz: 19093,
				},
				Thing {
					abc: 924024,
					xyz: 621,
				},
			],
		};

		println!("{}", to_string(&hey).unwrap());
	}
}
