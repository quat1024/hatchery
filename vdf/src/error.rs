use std::char::ParseCharError;
use std::fmt;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use serde::de;
use serde::ser;
use thiserror::Error;

/// Something that went wrong when serializing or deserializing.
#[derive(Error, Debug)]
pub enum VdfErr {
	/// An unspecified error from somewhere deeper in Serde machinery.
	#[error("{0}")]
	Message(String),
	/// When deserializing, the end of the file was encountered unexpectedly.
	#[error("Unexpected end of file")]
	EndOfFile,
	#[error("Expected a bool, but found {0}")]
	ParseBool(String),
	#[error("Could not parse integer: {0}")]
	ParseInt(ParseIntError),
	#[error("Could not parse floating-point: {0}")]
	ParseFloat(ParseFloatError),
	#[error("Could not parse char: {0}")]
	ParseChar(ParseCharError),
}

/// Alias for `Result<T, VdfErr>`. I don't really like Result aliases, but ok, here you go.
pub type Result<T> = std::result::Result<T, VdfErr>;

impl ser::Error for VdfErr {
	fn custom<T>(msg: T) -> Self
	where
		T: fmt::Display,
	{
		VdfErr::Message(msg.to_string())
	}
}

impl de::Error for VdfErr {
	fn custom<T>(msg: T) -> Self
	where
		T: fmt::Display,
	{
		VdfErr::Message(msg.to_string())
	}
}
