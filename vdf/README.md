# vdf

Serde for Valve's funny key-value store used across many of their products. Notably used in Hammer `.vmf` map files, but also in Portal 2 `editoritems.txt` and `.p2c` files, `gameinfo.txt` files, all sorts of things. I think it's a bit ad-hoc over there.

## Progress

* Ser: Pretty good, see below.
* De: Not implemented.

## Notes

The .vdf format is very basic so, shoving it into Serde's very complex data model required me to make up some things.

* `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `f32`, `i64`, `u64`, `char`, `i128` and `u128` if 128-bit number support enabled in Serde, and `str`
	* The usual.
* `bool`
	* The number `0` for false and `1` for true.
	* An option is available on the serializer to write them as the strings `"true"` and `"false"` instead.
* `None`, `()`, unit structs
	* The empty string.
* `Some(T)`
	* The same as `T`, i.e. the `Option` wrapper is discarded.
* Unit variants
	* By name, not by index
* Newtype structs
	* Are **NOT** transparent, unlike a lot of Serde data formats.
	* If `Y` serializes as `"k" "v"`, `struct Wrapper(Y);` serializes as `"Wrapper" { "k" "v" }`.
	* This is one way of creating a "top-level key" (e.g. the `ItemData` in `editoritems.txt`).
	* TODO, is this a good idea lol
* Maps
	* Keys and values treated as-is.
* Structs
	* Keys and values treated as-is, looks just like a map.
* Sequences
	* Serialized under keys `"0"`, `"1"`, `"2"`, `"3"` etc. (I had to make this up, but `editoritems.txt` does this sometimes, so there's some precedent.)
	* Some of Valve's files use what I'm calling an "inner entry tag". E.g. `editoritems.txt` is serialized with an outer `"ItemData"` block containing lots of `"Item"` blocks. The `"Item"` is what I'm calling the "inner entry tag".
	* To override this behavior, a macro is provided `vdf::named_seq_func`, that allows you to serialize any slice as a *map* with the same key repeated over-and-over, instead of this sequence.


Unsupported formats, for now, while I work out what to do with em:
* Bytes
* Tuples, tuple structs and variants
* Struct variants

## Known Issues

Some of Valve's files put keys and values of a struct at the same indentation level by introducing spaces and extra tabs to make everything line up. `vdf` makes no attempt to do that (I can't tell what the longest key will be ahead-of-time). It's theoretically possible by buffering the entire struct in-memory until I know there are no more keys but like, maaannnn

Also `gameinfo.txt` has some magic unquoted strings in `SearchPaths`, and I don't even know *what* to make of those.