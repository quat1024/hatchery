//! Macros to help out with representing your structures in the VDF format.

use serde::ser::SerializeMap;

/// Creates a helper function that serializes items of a slice under blocks of a given name, useful for "internally named" structures.
///
/// Since VDF doesn't have any special array syntax, Valve often uses repeated keys to accomplish this. This macro helps you specify the name of the repeated key.
///
/// ```text
/// todo put a doctested usage example in here, it keeps exploding though :(
/// ```
#[macro_export]
macro_rules! named_seq_func {
	( $func:ident $name:literal ) => {
		fn $func<S, T>(value: &[T], s: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
			T: serde::Serialize,
		{
			//TODO can I simply call the below function? Qualifying the name is hard...

			use serde::ser::SerializeMap;
			let mut map = s.serialize_map(Some(value.len()))?;
			for i in value {
				map.serialize_entry($name, &i)?;
			}
			map.end()
		}
	};
}

pub fn named_seq_serialize<S, T>(name: &str, slice: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
	T: serde::Serialize,
{
	let mut map = serializer.serialize_map(Some(slice.len()))?;
	for i in slice {
		map.serialize_entry(name, &i)?;
	}
	map.end()
}
